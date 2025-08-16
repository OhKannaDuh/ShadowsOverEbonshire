use rand::prelude::*;
use std::collections::VecDeque;

pub use wave_macros::WaveTiles;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    #[inline]
    pub fn dx(self) -> isize {
        match self {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        }
    }
    #[inline]
    pub fn dy(self) -> isize {
        match self {
            Direction::South => 1,
            Direction::North => -1,
            _ => 0,
        }
    }
}

pub trait Tile: Copy + Eq + 'static {
    fn all() -> &'static [Self];
}

pub trait HasSockets<S: 'static + Copy + Eq>: Tile {
    fn sockets(&self, dir: Direction) -> &'static [(S, u32)];
}

#[derive(Debug)]
pub struct Contradiction {
    pub at: (usize, usize),
}

/// WaveFunction holds only the grid/priors and RNG.
/// Socket type is introduced only where needed (e.g., collapse()).
pub struct WaveFunction<T: Tile> {
    w: usize,
    h: usize,
    rng: StdRng,
    priors: Vec<Option<T>>,
}

impl<T: Tile> WaveFunction<T> {
    pub fn new(size: [usize; 2], seed: u64) -> Self {
        let (w, h) = (size[0], size[1]);
        Self {
            w,
            h,
            rng: StdRng::seed_from_u64(seed),
            priors: vec![None; w * h],
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    /// Fix a tile at (x, y) prior to collapse.
    pub fn set(&mut self, pos: (usize, usize), tile: T) {
        let (x, y) = pos;
        assert!(x < self.w && y < self.h, "set out of bounds");

        let i = self.idx(x, y);
        self.priors[i] = Some(tile);
    }
}

// Methods that need sockets bring `S` into scope here.
impl<T> WaveFunction<T>
where
    T: Tile,
{
    pub fn collapse<S>(&mut self) -> Result<Vec<((usize, usize), T)>, Contradiction>
    where
        T: HasSockets<S>,
        S: 'static + Copy + Eq,
    {
        let n = T::all().len();

        // compat4[dir][i][j] == true if tile i is compatible with tile j in `dir`.
        let mut compat4: [Vec<Vec<bool>>; 4] = [
            vec![vec![false; n]; n],
            vec![vec![false; n]; n],
            vec![vec![false; n]; n],
            vec![vec![false; n]; n],
        ];

        use Direction::*;
        for (i, &ti) in T::all().iter().enumerate() {
            for (j, &tj) in T::all().iter().enumerate() {
                let ne = intersects(ti.sockets(East), tj.sockets(West));
                compat4[2][i][j] = ne; // East
                let nw = intersects(ti.sockets(West), tj.sockets(East));
                compat4[3][i][j] = nw; // West
                let nn = intersects(ti.sockets(North), tj.sockets(South));
                compat4[0][i][j] = nn; // North
                let ns = intersects(ti.sockets(South), tj.sockets(North));
                compat4[1][i][j] = ns; // South
            }
        }

        // Domains: per cell, bitset of possible tile indices (true = allowed)
        let mut dom: Vec<Vec<bool>> = vec![vec![true; n]; self.w * self.h];

        // Apply priors and propagate
        let mut q = VecDeque::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(t) = self.priors[self.idx(x, y)] {
                    let i = index_of(T::all(), &t).expect("tile must be in ALL");
                    let d = &mut dom[self.idx(x, y)];
                    for k in 0..n {
                        d[k] = k == i;
                    }
                    q.push_back((x, y));
                }
            }
        }
        propagate(self.w, self.h, &compat4, &mut dom, &mut q)?;

        // Observe until all cells are collapsed
        loop {
            // Find min-entropy cell (>1 choice)
            let mut best: Option<(usize, usize, usize)> = None; // (x,y,choices)
            for y in 0..self.h {
                for x in 0..self.w {
                    let d = &dom[self.idx(x, y)];
                    let choices = d.iter().filter(|&&b| b).count();
                    if choices > 1 {
                        match &mut best {
                            None => best = Some((x, y, choices)),
                            Some(b) if choices < b.2 => *b = (x, y, choices),
                            _ => {}
                        }
                    }
                }
            }
            let Some((bx, by, _)) = best else { break }; // done

            // Sample one allowed tile uniformly
            let allowed: Vec<usize> = dom[self.idx(bx, by)]
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| b.then_some(i))
                .collect();
            let pick = allowed[self.rng.gen_range(0..allowed.len())];
            for k in 0..n {
                dom[self.idx(bx, by)][k] = k == pick;
            }
            q.push_back((bx, by));
            propagate(self.w, self.h, &compat4, &mut dom, &mut q)?;
        }

        // Build output
        let mut out = Vec::with_capacity(self.w * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let d = &dom[self.idx(x, y)];
                let mut it = d.iter().enumerate().filter_map(|(i, &b)| b.then_some(i));
                let Some(i) = it.next() else {
                    return Err(Contradiction { at: (x, y) });
                };
                debug_assert!(it.next().is_none()); // collapsed
                out.push(((x, y), T::all()[i]));
            }
        }
        Ok(out)
    }
}

fn intersects<S: Copy + Eq>(a: &'static [(S, u32)], b: &'static [(S, u32)]) -> bool {
    for (x, _) in a {
        for (y, _) in b {
            if x == y {
                return true;
            }
        }
    }
    false
}

fn index_of<T: Copy + Eq>(xs: &[T], t: &T) -> Option<usize> {
    xs.iter().position(|x| x == t)
}

fn split_pair<'a>(
    dom: &'a mut [Vec<bool>],
    i: usize,
    j: usize,
) -> (&'a Vec<bool>, &'a mut Vec<bool>) {
    assert!(i != j);
    if i < j {
        let (left, right) = dom.split_at_mut(j);
        (&left[i], &mut right[0])
    } else {
        let (left, right) = dom.split_at_mut(i);
        (&right[0], &mut left[j])
    }
}

fn propagate(
    w: usize,
    h: usize,
    compat4: &[Vec<Vec<bool>>; 4],
    dom: &mut [Vec<bool>],
    q: &mut VecDeque<(usize, usize)>,
) -> Result<(), Contradiction> {
    use Direction::*;
    while let Some((x, y)) = q.pop_front() {
        let i_cur = y * w + x;

        for (dir, table) in [
            (North, &compat4[0]),
            (South, &compat4[1]),
            (East, &compat4[2]),
            (West, &compat4[3]),
        ] {
            let nx = x as isize + dir.dx();
            let ny = y as isize + dir.dy();
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if nx >= w || ny >= h {
                continue;
            }

            let i_neigh = ny * w + nx;
            if i_neigh == i_cur {
                continue;
            } // not possible for cardinal dirs, but safe

            // Borrow the two cells disjointly
            let (cur, neigh) = split_pair(dom, i_cur, i_neigh);

            // Keep only neighbor tiles compatible with at least one of current cell's tiles.
            let mut changed = false;
            for j in 0..neigh.len() {
                if !neigh[j] {
                    continue;
                }
                let mut ok = false;
                for (i, &ci) in cur.iter().enumerate() {
                    if ci && table[i][j] {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    neigh[j] = false;
                    changed = true;
                }
            }

            if changed {
                if neigh.iter().any(|&b| b) {
                    q.push_back((nx, ny));
                } else {
                    return Err(Contradiction { at: (nx, ny) });
                }
            }
        }
    }
    Ok(())
}

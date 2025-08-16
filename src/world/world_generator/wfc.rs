use crate::prelude::*;
use crate::world::world_generator::*;
use wave::{WaveFunction, WaveTiles};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SocketType {
    Core,
    Road,
    Wall,
    Grass,
    Building,
}

#[derive(WaveTiles, Copy, Clone, Debug, Eq, PartialEq)]
#[wave_socket(SocketType)]
enum Slate {
    #[socket_east ([(SocketType::Road, 1u32)])]
    #[socket_west ([(SocketType::Road, 1u32)])]
    #[socket_north([(SocketType::Grass, 4u32), (SocketType::Building, 1u32)])]
    #[socket_south([(SocketType::Grass, 4u32), (SocketType::Building, 1u32)])]
    RoadHorizontal,

    #[socket_vertical  ([(SocketType::Road, 1u32)])]
    #[socket_horizontal([(SocketType::Grass, 4u32), (SocketType::Building, 1u32)])]
    RoadVertical,
}

const WORLD_WIDTH: f32 = 48.0; // (chunks)
const WORLD_HEIGHT: f32 = 48.0; // (chunks)

const CHUNK_DIMENSION: f32 = 64.0; // (tiles)
const SLATE_DIMENSION: f32 = 8.0; // (tiles)
const TILE_DIMENSION: f32 = 32.0; // (pixels)

const WORLD_WIDTH_PX: f32 = WORLD_WIDTH * CHUNK_DIMENSION * TILE_DIMENSION;
const WORLD_HEIGHT_PX: f32 = WORLD_HEIGHT * CHUNK_DIMENSION * TILE_DIMENSION;
const SLATE_DIMENSION_PX: f32 = SLATE_DIMENSION * TILE_DIMENSION;

const WORLD_WIDTH_SLATES_F: f32 = WORLD_WIDTH_PX / SLATE_DIMENSION_PX;
const WORLD_HEIGHT_SLATES_F: f32 = WORLD_HEIGHT_PX / SLATE_DIMENSION_PX;

// If you prefer const ints:
const WORLD_WIDTH_SLATES: usize = WORLD_WIDTH_SLATES_F as usize;
const WORLD_HEIGHT_SLATES: usize = WORLD_HEIGHT_SLATES_F as usize;

#[add_system(
    schedule = OnEnter(GameState::GeneratingMap),
    plugin = WorldPlugin,
    after = sample_world_features
)]
fn wave_function_collapse(mut next_state: ResMut<NextState<GameState>>) {
    let seed: u64 = 23_534_536_336_534;

    // Build the wave (note: WaveFunction only takes `Slate` now)
    let mut wave = WaveFunction::<Slate>::new([WORLD_WIDTH_SLATES, WORLD_HEIGHT_SLATES], seed);

    let center_x = WORLD_WIDTH_SLATES / 2;
    let center_y = WORLD_HEIGHT_SLATES / 2;

    // Set starting tiles (priors)
    wave.set((center_x, center_y), Slate::RoadHorizontal);
    wave.set((center_x + 1, center_y), Slate::RoadHorizontal);

    // Collapse. Because HasSockets is generic in `S`, we specify the socket type here.
    let tiles: Vec<((usize, usize), Slate)> =
        wave.collapse::<SocketType>().expect("WFC contradiction");

    // If you actually want sockets per cell rather than the chosen Slate, map them here.
    // You need a policy for which side to take; here's an example using 'North' and
    // falling back to SocketType::Core if empty:
    /*
    use wave::Direction;
    let tiles_sockets: Vec<((usize, usize), SocketType)> = tiles.iter().map(|(p, t)| {
        let s = t.sockets(Direction::North).get(0).copied().unwrap_or(SocketType::Core);
        (*p, s)
    }).collect();
    */

    // save_slate_image(&space, "wfc_generation.png").expect("Failed to save slate image");

    next_state.set(GameState::InGame);
}

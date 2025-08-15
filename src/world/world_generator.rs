use crate::prelude::*;
use crate::world::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::world::biome::*;

const DEFAULT_SEED: u32 = 123456789;

pub struct Point {
    pub x: i32,
    pub y: i32,

    pub temperature: f32,
    pub temperature_level: u32,

    pub humidity: f32,
    pub humidity_level: u32,

    pub continentalness: f32,
    pub continentalness_level: u32,

    pub erosion: f32,
    pub erosion_level: u32,

    pub weirdness: f32,
    pub is_weird: bool,

    pub peaks_and_valleys: f32,
    pub peaks_and_valleys_level: u32,
}

impl Point {
    pub fn new(x: i32, y: i32, generator: &WorldGenerator) -> Self {
        let temperature = generator.temperature_at(x, y);
        let humidity = generator.humidity_at(x, y);
        let continentalness = generator.continentalness_at(x, y);
        let erosion = generator.erosion_at(x, y);
        let weirdness = generator.weirdness_at(x, y);
        let peaks_and_valleys = 1.0 - ((3.0 * weirdness.abs()) - 2.0).abs();

        Point {
            x,
            y,
            temperature,
            temperature_level: match temperature {
                t if t < -0.45 => 0,
                t if t < -0.15 => 1,
                t if t < 0.2 => 2,
                t if t < 0.55 => 3,
                _ => 4,
            },
            humidity,
            humidity_level: match humidity {
                h if h < -0.35 => 0,
                h if h < -0.1 => 1,
                h if h < 0.1 => 2,
                h if h < 0.3 => 3,
                _ => 4,
            },
            continentalness,
            continentalness_level: match continentalness {
                // c if c < -0.455 => 0, // Deep ocean
                // c if c < -0.19 => 1,  // Ocean
                // c if c < -0.11 => 2,  // Coast
                // c if c < 0.03 => 3,   // Near-inland
                // c if c < 0.3 => 4,    // Mid-inland
                // _ => 5,               // Far-inland
                c if c < -0.55 => 0, // Deep ocean (rarer)
                c if c < -0.28 => 1, // Ocean (rarer)
                c if c < -0.05 => 2, // Coast (a bit wider shores)
                c if c < 0.05 => 3,  // Near-inland
                c if c < 0.35 => 4,  // Mid-inland
                _ => 5,              // Far-inland
            },
            erosion,
            erosion_level: match erosion {
                e if e < -0.78 => 0,
                e if e < -0.375 => 1,
                e if e < -0.2225 => 2,
                e if e < 0.05 => 3,
                e if e < 0.45 => 4,
                e if e < 0.55 => 5,
                _ => 6,
            },
            weirdness,
            is_weird: weirdness > 0.0,
            peaks_and_valleys,
            peaks_and_valleys_level: match peaks_and_valleys {
                v if v < -0.85 => 0, // Valleys
                v if v < -0.2 => 1,  // Low
                v if v < 0.2 => 2,   // Mid
                v if v < 0.7 => 3,   // High
                _ => 4,              // Peaks
            },
        }
    }

    pub fn get_biome(&self) -> Biome {
        pick_biome(self)
    }
}

#[derive(Resource, Debug)]
#[insert_resource(plugin = WorldPlugin)]
pub struct WorldGeneratationConfig {
    pub seed: u32,
    pub chunk_width: u32,
    pub chunk_height: u32,
    pub tile_size: f32,
    pub load_radius: i32,

    pub chunk_width_i32: i32,
    pub chunk_height_i32: i32,

    pub tilemap_size: TilemapSize,
    pub tilemap_tile_size: TilemapTileSize,
    pub tilemap_grid_size: TilemapGridSize,
}

impl Default for WorldGeneratationConfig {
    fn default() -> Self {
        let chunk_width = 64;
        let chunk_height = 64;
        let tile_size = 32.0;

        let seed = rand::thread_rng().gen_range(0..=u32::MAX);
        // let seed = DEFAULT_SEED;

        WorldGeneratationConfig {
            seed: seed,
            chunk_width,
            chunk_height,
            tile_size,
            load_radius: 32,

            chunk_width_i32: 64,
            chunk_height_i32: 64,

            tilemap_size: TilemapSize::new(chunk_width, chunk_height),
            tilemap_tile_size: TilemapTileSize::new(tile_size, tile_size),
            tilemap_grid_size: TilemapGridSize::new(tile_size, tile_size),
        }
    }
}

#[derive(Resource, Debug)]
#[insert_resource(plugin = WorldPlugin)]
pub struct WorldGenerator {
    temperature_noise: Perlin,
    humidity_noise: Perlin,
    continentalness_noise: Perlin,
    erosion_noise: Perlin,
    weirdness_noise: Perlin,
    land_mask_noise: Perlin,

    c_off: (f64, f64),
    t_off: (f64, f64),
    h_off: (f64, f64),
    e_off: (f64, f64),
    w_off: (f64, f64),

    c_rot: f64,
    t_rot: f64,
    h_rot: f64,
    e_rot: f64,
    w_rot: f64,

    equator_offset: f64,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        WorldGenerator::new(WorldGeneratationConfig::default())
    }
}

impl WorldGenerator {
    pub fn new(config: WorldGeneratationConfig) -> Self {
        let equator_noise = Perlin::new(config.seed.wrapping_add(4));
        let raw = equator_noise.get([0.0]);
        let equator_offset = raw * config.chunk_height as f64 * 16.0;

        let seed_u64 = config.seed as u64;
        let mk = |n| {
            (((seed_u64.wrapping_mul(6364136223846793005).wrapping_add(n)) % 10_000_000) as f64)
                / 7.0
        };

        fn hash_u64(seed: u64, k: u64) -> u64 {
            // simple SplitMix64-ish scrambler
            let mut z = seed.wrapping_add(k).wrapping_add(0x9E3779B97F4A7C15);
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        }

        fn f64_01(seed: u64, k: u64) -> f64 {
            // uniform in [0,1)
            (hash_u64(seed, k) as f64) / (u64::MAX as f64)
        }

        fn offset_pair(seed: u64, kx: u64, ky: u64, magnitude: f64) -> (f64, f64) {
            // big, non-integer offsets to avoid lattice origin
            (
                (f64_01(seed, kx) * magnitude),
                (f64_01(seed, ky) * magnitude),
            )
        }

        fn angle(seed: u64, k: u64) -> f64 {
            // radians in [0, 2π)
            f64_01(seed, k) * std::f64::consts::TAU
        }

        WorldGenerator {
            temperature_noise: Perlin::new(config.seed),
            humidity_noise: Perlin::new(config.seed.wrapping_add(1)),
            continentalness_noise: Perlin::new(config.seed.wrapping_add(2)),
            erosion_noise: Perlin::new(config.seed.wrapping_add(3)),
            weirdness_noise: Perlin::new(config.seed.wrapping_add(4)),

            land_mask_noise: Perlin::new(config.seed.wrapping_add(42)),

            c_off: (mk(11), mk(12)),
            t_off: (mk(21), mk(22)),
            h_off: (mk(31), mk(32)),
            e_off: (mk(41), mk(42)),
            w_off: (mk(51), mk(52)),

            c_rot: angle(seed_u64, 61),
            t_rot: angle(seed_u64, 62),
            h_rot: angle(seed_u64, 63),
            e_rot: angle(seed_u64, 64),
            w_rot: angle(seed_u64, 65),

            equator_offset,
        }
    }

    fn fractal_noise(
        noise: &Perlin,
        x: f64,
        y: f64,
        octaves: usize,
        persistence: f32,
        lacunarity: f64,
    ) -> f32 {
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        let mut total = 0.0;

        for _ in 0..octaves {
            let val = noise.get([x * frequency, y * frequency]) as f32;
            total += val * amplitude;
            max_value += amplitude;

            amplitude *= persistence;
            frequency *= lacunarity;
        }

        total / max_value
    }

    fn rotate(x: f64, y: f64, theta: f64) -> (f64, f64) {
        let (s, c) = theta.sin_cos();
        (x * c - y * s, x * s + y * c)
    }

    pub fn temperature_at(&self, x: i32, y: i32) -> f32 {
        let scale = 0.001;
        let (mut nx, mut ny) = (
            (x as f64 + self.t_off.0) * scale,
            (y as f64 + self.equator_offset + self.t_off.1) * scale,
        );
        (nx, ny) = Self::rotate(nx, ny, self.t_rot);
        Self::fractal_noise(&self.temperature_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn humidity_at(&self, x: i32, y: i32) -> f32 {
        let scale = 0.001;
        let (mut nx, mut ny) = (
            (x as f64 + self.h_off.0) * scale,
            (y as f64 + self.equator_offset + self.h_off.1) * scale,
        );
        (nx, ny) = Self::rotate(nx, ny, self.h_rot);
        Self::fractal_noise(&self.humidity_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn continentalness_at(&self, x: i32, y: i32) -> f32 {
        let scale = 0.001;
        let (mut nx, mut ny) = (
            (x as f64 + self.c_off.0) * scale,
            (y as f64 + self.c_off.1) * scale,
        );
        (nx, ny) = Self::rotate(nx, ny, self.c_rot);

        let base = Self::fractal_noise(&self.continentalness_noise, nx, ny, 4, 0.5, 2.0);

        // Low-frequency mask to bias toward land
        let mask_scale = 0.0002;
        let (mut mx, mut my) = (
            (x as f64 + self.c_off.0) * mask_scale,
            (y as f64 + self.c_off.1) * mask_scale,
        );
        (mx, my) = Self::rotate(mx, my, self.c_rot * 0.5); // mask rotation too
        let mask = Self::fractal_noise(&self.land_mask_noise, mx, my, 3, 0.5, 2.0);

        let mask01 = (mask + 1.0) * 0.5;
        let boosted = base + (mask01 - 0.5) * 0.4; // +/-0.2 push

        boosted.clamp(-1.0, 1.0)
    }

    pub fn erosion_at(&self, x: i32, y: i32) -> f32 {
        let scale = 0.001;
        let (mut nx, mut ny) = (
            (x as f64 + self.e_off.0) * scale,
            (y as f64 + self.e_off.1) * scale,
        );
        (nx, ny) = Self::rotate(nx, ny, self.e_rot);
        Self::fractal_noise(&self.erosion_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn weirdness_at(&self, x: i32, y: i32) -> f32 {
        let scale = 0.001;
        let (mut nx, mut ny) = (
            (x as f64 + self.w_off.0) * scale,
            (y as f64 + self.w_off.1) * scale,
        );
        (nx, ny) = Self::rotate(nx, ny, self.w_rot);
        Self::fractal_noise(&self.weirdness_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn generate_chunk(
        &self,
        commands: &mut Commands,
        chunk_x: i32,
        chunk_y: i32,
        config: &WorldGeneratationConfig,
        image: Handle<Image>,
    ) -> Entity {
        let tilemap = commands.spawn_empty().id();

        let mut storage = TileStorage::empty(config.tilemap_size);

        for y in 0..config.chunk_height {
            for x in 0..config.chunk_width {
                let tile_pos = TilePos { x, y };

                let world_x = chunk_x * config.chunk_width_i32 + tile_pos.x as i32;
                let world_y = chunk_y * config.chunk_height_i32 + tile_pos.y as i32;

                let point = Point::new(world_x, world_y, self);

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(1), // change to get from biome
                        tilemap_id: TilemapId(tilemap),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap).add_child(tile_entity);
                storage.set(&tile_pos, tile_entity);
            }
        }

        commands.entity(tilemap).insert(TilemapBundle {
            grid_size: config.tilemap_grid_size,
            size: TilemapSize {
                x: config.chunk_width,
                y: config.chunk_height,
            },
            storage,
            texture: TilemapTexture::Single(image.clone()),
            tile_size: config.tilemap_tile_size,
            transform: Transform::from_translation(Vec3::new(
                (chunk_x * config.chunk_width_i32) as f32 * config.tilemap_tile_size.x,
                (chunk_y * config.chunk_height_i32) as f32 * config.tilemap_tile_size.y,
                -64.0,
            )),
            ..Default::default()
        });

        tilemap
    }

    pub fn get_point(&self, world_x: i32, world_y: i32) -> Point {
        Point::new(world_x, world_y, self)
    }
}

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

        WorldGenerator {
            temperature_noise: Perlin::new(config.seed),
            humidity_noise: Perlin::new(config.seed.wrapping_add(1)),
            continentalness_noise: Perlin::new(config.seed.wrapping_add(2)),
            erosion_noise: Perlin::new(config.seed.wrapping_add(3)),
            weirdness_noise: Perlin::new(config.seed.wrapping_add(4)),

            land_mask_noise: Perlin::new(config.seed.wrapping_add(42)),

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

    pub fn temperature_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = (y as f64 + self.equator_offset) * 0.001;

        WorldGenerator::fractal_noise(&self.temperature_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn humidity_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = (y as f64 + self.equator_offset) * 0.001;

        WorldGenerator::fractal_noise(&self.humidity_noise, nx, ny, 4, 0.5, 2.0)
    }

    // pub fn continentalness_at(&self, x: i32, y: i32) -> f32 {
    //     let nx = x as f64 * 0.001;
    //     let ny = y as f64 * 0.001;

    //     WorldGenerator::fractal_noise(&self.continentalness_noise, nx, ny, 4, 0.5, 2.0)
    // }

    pub fn continentalness_at(&self, x: i32, y: i32) -> f32 {
        let base = WorldGenerator::fractal_noise(
            &self.continentalness_noise,
            x as f64 * 0.001,
            y as f64 * 0.001,
            4,
            0.5,
            2.0,
        );

        let mask = WorldGenerator::fractal_noise(
            &self.land_mask_noise,
            x as f64 * 0.0002,
            y as f64 * 0.0002,
            3,
            0.5,
            2.0,
        );

        let mask01 = (mask + 1.0) * 0.5;
        let boosted = base + (mask01 - 0.5) * 0.4; // +/-0.2 push

        boosted.clamp(-1.0, 1.0)
    }

    pub fn erosion_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = y as f64 * 0.001;

        WorldGenerator::fractal_noise(&self.erosion_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn weirdness_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = y as f64 * 0.001;

        WorldGenerator::fractal_noise(&self.weirdness_noise, nx, ny, 4, 0.5, 2.0)
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

use crate::prelude::*;
use crate::world::*;
use bevy_simple_tilemap::Tile;
use noise::{NoiseFn, Perlin};

const DEFAULT_SEED: u32 = 123456789;

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
    pub chunk_width_usize: usize,
    pub chunk_height_usize: usize,
}

impl Default for WorldGeneratationConfig {
    fn default() -> Self {
        WorldGeneratationConfig {
            seed: DEFAULT_SEED,
            chunk_width: 64,
            chunk_height: 64,
            tile_size: 32.0,
            load_radius: 2,

            chunk_width_i32: 64,
            chunk_height_i32: 64,
            chunk_width_usize: 64,
            chunk_height_usize: 64,
        }
    }
}

#[derive(Resource, Debug)]
#[insert_resource(plugin = WorldPlugin)]
pub struct WorldGenerator {
    noise: Perlin,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        WorldGenerator::new(WorldGeneratationConfig::default())
    }
}

impl WorldGenerator {
    pub fn new(config: WorldGeneratationConfig) -> Self {
        WorldGenerator {
            noise: Perlin::new(config.seed),
        }
    }

    pub fn height_at(&self, world_x: i32, world_y: i32) -> f32 {
        let nx = world_x as f64 * 0.001;
        let ny = world_y as f64 * 0.001;

        let base = self.noise.get([nx, ny]) as f32;

        let mut height = (base + 1.0) / 2.0;

        let flat_radius = 128.0;
        let dist = ((world_x.pow(2) + world_y.pow(2)) as f32).sqrt();
        if dist < flat_radius {
            let t = dist / flat_radius;
            let flat_val = 0.5;
            height = height * t + flat_val * (1.0 - t);
        }

        let sea_level = 0.3;
        if height < sea_level {
            height = sea_level - ((sea_level - height) * 0.5);
        }

        height.clamp(0.0, 1.0)
    }

    pub fn tile_for_height(&self, h: f32) -> u32 {
        match h {
            0.0..=0.3 => 3,  // water
            0.3..=0.4 => 2,  // sand
            0.4..=0.45 => 1, // dirt
            0.45..=0.7 => 0, // grass
            _ => 4,          // dense foiliage
        }
    }

    pub fn generate_chunk(
        &self,
        chunk_x: i32,
        chunk_y: i32,
        config: &Res<WorldGeneratationConfig>,
    ) -> Vec<(IVec3, Option<Tile>)> {
        let offset_x = chunk_x * config.chunk_width_i32;
        let offset_y = chunk_y * config.chunk_height_i32;

        let mut tiles = Vec::with_capacity(config.chunk_width_usize * config.chunk_height_usize);

        for y in 0..config.chunk_height {
            for x in 0..config.chunk_width {
                let wx = offset_x + x as i32;
                let wy = offset_y + y as i32;

                let h = self.height_at(wx, wy);
                let sprite_index = self.tile_for_height(h);

                tiles.push((
                    ivec3(wx, wy, 0),
                    Some(Tile {
                        sprite_index,
                        color: Color::WHITE,
                        ..default()
                    }),
                ));
            }
        }

        tiles
    }
}

use crate::prelude::*;
use crate::world::*;
use bevy_ecs_tilemap::prelude::*;
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

    pub tilemap_size: TilemapSize,
    pub tilemap_tile_size: TilemapTileSize,
    pub tilemap_grid_size: TilemapGridSize,
}

impl Default for WorldGeneratationConfig {
    fn default() -> Self {
        let chunk_width = 64;
        let chunk_height = 64;
        let tile_size = 32.0;

        WorldGeneratationConfig {
            seed: DEFAULT_SEED,
            chunk_width,
            chunk_height,
            tile_size,
            load_radius: 1,

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

        let height = (base + 1.0) / 2.0;

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

                let tile_id = self.tile_for_height(self.height_at(world_x, world_y));

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(tile_id),
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
}

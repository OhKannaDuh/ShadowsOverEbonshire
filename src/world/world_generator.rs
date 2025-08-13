use crate::prelude::*;
use crate::world::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

const DEFAULT_SEED: u32 = 123456789;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Climate {
    Tropical,
    Arid,
    Temperate,
    Boreal,
    Polar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    // Tropical biomes
    Rainforest,
    Savannah,
    TropicalSeasonalForest,

    // Arid biomes
    Desert,
    SemiDesert,
    XericShrubland,

    // Temperate biomes
    Grassland,
    DeciduousForest,
    TemperateRainforest,
    Mediterranean,

    // Boreal biomes
    Taiga,
    BorealForest,

    // Polar biomes
    Tundra,
    IceSheet,

    // Special biomes
    Mountain,
    Swamp,
    River,

    Ocean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileId {
    Rainforest,
    Savannah,
    TropicalSeasonalForest,

    Desert,
    SemiDesert,
    XericShrubland,

    Grassland,
    DeciduousForest,
    TemperateRainforest,
    Mediterranean,

    Taiga,
    BorealForest,

    Tundra,
    IceSheet,

    Mountain,
    Swamp,
    River,
    Snow,

    Beach,
    ShallowOcean,
    Ocean,
    DeepOcean,
}

pub struct Point {
    pub x: i32,
    pub y: i32,

    pub elevation: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub moisture: f32,

    pub climate: Climate,
    pub biome: Biome,
    pub tile_id: TileId,
}

impl Point {
    pub fn new(x: i32, y: i32, generator: &WorldGenerator) -> Self {
        let elevation = generator.elevation_at(x, y);
        let temperature = generator.temperature_at(x, y);
        let humidity = generator.humidity_at(x, y);
        let moisture = generator.moisture_at(x, y);

        let climate = Point::determine_climate(temperature, humidity);
        let biome = Point::determine_biome(climate, elevation, moisture);
        let tile_id = Point::determine_tile_id(biome, elevation);

        Point {
            x,
            y,

            elevation,
            temperature,
            humidity,
            moisture,

            climate,
            biome,
            tile_id,
        }
    }

    fn determine_climate(temperature: f32, humidity: f32) -> Climate {
        if temperature > 0.7 && humidity > 0.7 {
            Climate::Tropical
        } else if temperature > 0.7 && humidity < 0.3 {
            Climate::Arid
        } else if temperature > 0.3 && temperature <= 0.7 {
            Climate::Temperate
        } else if temperature > 0.1 {
            Climate::Boreal
        } else {
            Climate::Polar
        }
    }

    fn determine_biome(climate: Climate, elevation: f32, moisture: f32) -> Biome {
        if elevation > 0.8 {
            return Biome::Mountain;
        }

        if elevation < 0.35 {
            return Biome::Ocean;
        }

        match climate {
            Climate::Tropical => {
                if moisture > 0.8 {
                    Biome::Rainforest
                } else if moisture > 0.5 {
                    Biome::TropicalSeasonalForest
                } else {
                    Biome::Savannah
                }
            }
            Climate::Arid => {
                if moisture < 0.2 {
                    Biome::Desert
                } else if moisture < 0.4 {
                    Biome::SemiDesert
                } else {
                    Biome::XericShrubland
                }
            }
            Climate::Temperate => {
                if moisture < 0.3 {
                    Biome::Grassland
                } else if moisture < 0.6 {
                    Biome::Mediterranean
                } else if moisture < 0.8 {
                    Biome::DeciduousForest
                } else {
                    Biome::TemperateRainforest
                }
            }
            Climate::Boreal => {
                if moisture < 0.5 {
                    Biome::Taiga
                } else {
                    Biome::BorealForest
                }
            }
            Climate::Polar => {
                if moisture < 0.3 {
                    Biome::Tundra
                } else {
                    Biome::IceSheet
                }
            }
        }
    }

    fn determine_tile_id(biome: Biome, elevation: f32) -> TileId {
        match biome {
            Biome::Mountain => {
                if elevation > 0.95 {
                    TileId::Snow
                } else {
                    TileId::Mountain
                }
            }

            Biome::Ocean => {
                if elevation < 0.3 {
                    TileId::ShallowOcean
                } else if elevation < 0.2 {
                    TileId::Ocean
                } else if elevation < 0.15 {
                    TileId::DeepOcean
                } else {
                    TileId::Beach
                }
            }

            Biome::River => TileId::River,
            Biome::Swamp => TileId::Swamp,

            Biome::Rainforest => TileId::Rainforest,
            Biome::Savannah => TileId::Savannah,
            Biome::TropicalSeasonalForest => TileId::TropicalSeasonalForest,

            Biome::Desert => TileId::Desert,
            Biome::SemiDesert => TileId::SemiDesert,
            Biome::XericShrubland => TileId::XericShrubland,

            Biome::Grassland => TileId::Grassland,
            Biome::DeciduousForest => TileId::DeciduousForest,
            Biome::TemperateRainforest => TileId::TemperateRainforest,
            Biome::Mediterranean => TileId::Mediterranean,

            Biome::Taiga => TileId::Taiga,
            Biome::BorealForest => TileId::BorealForest,

            Biome::Tundra => TileId::Tundra,
            Biome::IceSheet => TileId::IceSheet,
        }
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
    elevation_noise: Perlin,
    temperature_noise: Perlin,
    humidity_noise: Perlin,
    moisture_noise: Perlin,
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
            elevation_noise: Perlin::new(config.seed),
            temperature_noise: Perlin::new(config.seed.wrapping_add(1)),
            humidity_noise: Perlin::new(config.seed.wrapping_add(2)),
            moisture_noise: Perlin::new(config.seed.wrapping_add(3)),
            equator_offset,
        }
    }

    fn smoothed_elevation_at(&self, x: i32, y: i32) -> f32 {
        let mut sum = 0.0;
        let mut count = 0.0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                sum += self.elevation_at(x + dx, y + dy);
                count += 1.0;
            }
        }
        sum / count
    }

    fn latitude_gradient(&self, y: i32) -> f32 {
        // adjust y by offset and scale down
        let y_adj = (y as f64 + self.equator_offset) * 0.001;
        // take absolute value and clamp between 0 and 1
        let dist = y_adj.abs().min(1.0);
        // invert so 1.0 at equator, 0.0 at pole
        (1.0 - dist) as f32
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

        (total / max_value + 1.0) * 0.5
    }

    pub fn elevation_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = y as f64 * 0.001;

        WorldGenerator::fractal_noise(&self.elevation_noise, nx, ny, 5, 0.5, 2.0)
    }

    pub fn temperature_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = (y as f64 + self.equator_offset) * 0.001;

        let noise_temp =
            WorldGenerator::fractal_noise(&self.temperature_noise, nx, ny, 4, 0.5, 2.0);
        let lat_grad = self.latitude_gradient(y);

        noise_temp * 0.7 + lat_grad * 0.3
    }

    pub fn humidity_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = y as f64 * 0.001;

        let lat_grad = self.latitude_gradient(y);
        let noise_hum = WorldGenerator::fractal_noise(&self.humidity_noise, nx, ny, 4, 0.5, 2.0);
        noise_hum * 0.8 + lat_grad * 0.2
        // WorldGenerator::fractal_noise(&self.humidity_noise, nx, ny, 4, 0.5, 2.0)
    }

    pub fn moisture_at(&self, x: i32, y: i32) -> f32 {
        let nx = x as f64 * 0.001;
        let ny = y as f64 * 0.001;

        WorldGenerator::fractal_noise(&self.moisture_noise, nx, ny, 4, 0.5, 2.0)
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
                        texture_index: TileTextureIndex(point.tile_id as u32),
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

use bevy::platform::collections::HashMap;
use bevy::time::common_conditions::on_timer;
use bevy_ecs_tilemap::tiles::TilePos;
use std::time::Duration;

use crate::player::*;
use crate::prelude::*;
use crate::world::*;

#[derive(Resource, Default)]
#[insert_resource(plugin = WorldPlugin)]
struct ChunkManager(HashMap<(i32, i32), Entity>);

fn world_pos_to_chunk_coords(pos: Vec2, config: &WorldGeneratationConfig) -> (i32, i32) {
    let chunk_width_world = config.chunk_width as f32 * config.tile_size;
    let chunk_height_world = config.chunk_height as f32 * config.tile_size;

    let chunk_x = (pos.x / chunk_width_world).floor() as i32;
    let chunk_y = (pos.y / chunk_height_world).floor() as i32;
    (chunk_x, chunk_y)
}

#[add_system(
    schedule = Update,
    plugin = WorldPlugin,
    run_if = on_timer(Duration::from_millis(500)),
    run_if = in_state(GameState::InGame)
)]
fn load_near_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut chunks: ResMut<ChunkManager>,
    generator: Res<WorldGenerator>,
    config: Res<WorldGeneratationConfig>,
    asset_server: Res<AssetServer>,
) {
    let player_transform = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let (player_chunk_x, player_chunk_y) =
        world_pos_to_chunk_coords(player_transform.translation.truncate(), &config);

    let tilemap_image = asset_server.load("textures/tiles/tilemap.png");

    for dy in -config.load_radius..=config.load_radius {
        for dx in -config.load_radius..=config.load_radius {
            let chunk_coord = (player_chunk_x + dx, player_chunk_y + dy);

            if chunks.0.contains_key(&chunk_coord) {
                continue;
            }

            let chunk_entity = generator.generate_chunk(
                &mut commands,
                chunk_coord.0,
                chunk_coord.1,
                &config,
                tilemap_image.clone(),
            );

            chunks.0.insert(chunk_coord, chunk_entity);
            info!(" - Generated chunk at {:?}", chunk_coord);
        }
    }
}

#[add_system(
    schedule = Update,
    plugin = WorldPlugin,
    run_if = on_timer(Duration::from_millis(500)),
    run_if = in_state(GameState::InGame),
    after = load_near_chunks)]
fn unload_far_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut chunks: ResMut<ChunkManager>,
    config: Res<WorldGeneratationConfig>,
) {
    let player_transform = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    let (player_chunk_x, player_chunk_y) =
        world_pos_to_chunk_coords(player_transform.translation.truncate(), &config);

    let unload_distance = config.load_radius * 2;

    // Find chunks to unload (too far away)
    let to_unload: Vec<(i32, i32)> = chunks
        .0
        .iter()
        .filter(|&(&chunk_coord, _)| {
            let (cx, cy) = chunk_coord;
            (cx - player_chunk_x).abs() > unload_distance
                || (cy - player_chunk_y).abs() > unload_distance
        })
        .map(|(&chunk_coord, _)| chunk_coord)
        .collect();

    for chunk_coord in to_unload {
        if let Some(entity) = chunks.0.remove(&chunk_coord) {
            commands.entity(entity).despawn();
            info!(" - Unloaded chunk at {:?}", chunk_coord);
        }
    }
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::InGame))]
fn draw_chunk_grid(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
    config: Res<WorldGeneratationConfig>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    let (player_chunk_x, player_chunk_y) = world_pos_to_chunk_coords(player_pos, &config);

    let chunk_width_world = config.chunk_width as f32 * config.tile_size;
    let chunk_height_world = config.chunk_height as f32 * config.tile_size;

    let draw_radius = config.load_radius + 1;

    let min_chunk_x = player_chunk_x - draw_radius;
    let max_chunk_x = player_chunk_x + draw_radius;
    let min_chunk_y = player_chunk_y - draw_radius;
    let max_chunk_y = player_chunk_y + draw_radius;

    let offset = -config.tile_size / 2.0;

    let min_x = min_chunk_x as f32 * chunk_width_world + offset;
    let max_x = (max_chunk_x + 1) as f32 * chunk_width_world + offset;
    let min_y = min_chunk_y as f32 * chunk_height_world + offset;
    let max_y = (max_chunk_y + 1) as f32 * chunk_height_world + offset;

    for chunk_x in min_chunk_x..=max_chunk_x + 1 {
        let world_x = chunk_x as f32 * chunk_width_world + offset;
        gizmos.line(
            Vec3::new(world_x, min_y, 100.0),
            Vec3::new(world_x, max_y, 100.0),
            Color::srgb(1.0, 1.0, 0.0),
        );
    }

    // Horizontal lines
    for chunk_y in min_chunk_y..=max_chunk_y + 1 {
        let world_y = chunk_y as f32 * chunk_height_world + offset;
        gizmos.line(
            Vec3::new(min_x, world_y, 100.0),
            Vec3::new(max_x, world_y, 100.0),
            Color::srgb(1.0, 1.0, 0.0),
        );
    }
}
#[add_system(schedule = OnEnter(GameState::InGame), plugin = WorldPlugin)]
fn generate_world_image(config: Res<WorldGeneratationConfig>, generator: Res<WorldGenerator>) {
    use image::{Rgba, RgbaImage};

    info!("Generating world image...");

    let num_chunks_x = 32_i32; // number of chunks in +x and -x directions
    let num_chunks_y = 32_i32; // same for y

    // total chunks wide/tall
    let total_chunks_x = num_chunks_x * 2 + 1;
    let total_chunks_y = num_chunks_y * 2 + 1;

    // Image size in pixels = number of tiles horizontally and vertically
    let map_width = config.chunk_width * total_chunks_x as u32;
    let map_height = config.chunk_height * total_chunks_y as u32;

    let mut map_image = RgbaImage::new(map_width, map_height);

    fn tile_id_to_color(tile_id: TileId) -> Rgba<u8> {
        match tile_id {
            TileId::Rainforest => Rgba([0, 100, 0, 255]), // Dark green
            TileId::Savannah => Rgba([189, 183, 107, 255]), // Dark khaki
            TileId::TropicalSeasonalForest => Rgba([34, 139, 34, 255]), // Forest green

            TileId::Desert => Rgba([237, 201, 175, 255]), // Sandy beige
            TileId::SemiDesert => Rgba([210, 180, 140, 255]), // Tan
            TileId::XericShrubland => Rgba([160, 82, 45, 255]), // Sienna

            TileId::Grassland => Rgba([124, 252, 0, 255]), // Lawn green
            TileId::DeciduousForest => Rgba([34, 139, 34, 255]), // Forest green
            TileId::TemperateRainforest => Rgba([0, 100, 0, 255]), // Dark green
            TileId::Mediterranean => Rgba([107, 142, 35, 255]), // Olive drab

            TileId::Taiga => Rgba([46, 139, 87, 255]), // Sea green
            TileId::BorealForest => Rgba([0, 128, 0, 255]), // Green

            TileId::Tundra => Rgba([176, 196, 222, 255]), // Light steel blue
            TileId::IceSheet => Rgba([240, 248, 255, 255]), // Alice blue (icy)

            TileId::Mountain => Rgba([139, 137, 137, 255]), // Gray
            TileId::Swamp => Rgba([47, 79, 47, 255]),       // Dark slate gray
            TileId::River => Rgba([30, 144, 255, 255]),     // Dodger blue

            TileId::Water => Rgba([0, 0, 255, 255]), // Blue
            TileId::Sand => Rgba([244, 164, 96, 255]), // Sandy brown
            TileId::Snow => Rgba([255, 250, 250, 255]), // Snow white
        }
    }

    for chunk_y in -num_chunks_y..=num_chunks_y {
        for chunk_x in -num_chunks_x..=num_chunks_x {
            info!("Generating chunk at ({}, {})", chunk_x, chunk_y);
            for tile_y in 0..config.chunk_height {
                for tile_x in 0..config.chunk_width {
                    // Calculate world tile coords
                    let world_x = chunk_x * config.chunk_width as i32 + tile_x as i32;
                    let world_y = chunk_y * config.chunk_height as i32 + tile_y as i32;

                    let point = Point::new(world_x, world_y, &generator);

                    let color = tile_id_to_color(point.tile_id);

                    // Calculate pixel coordinates in the image
                    let px = (chunk_x + num_chunks_x) as u32 * config.chunk_width + tile_x;
                    // Invert y to draw top-to-bottom (optional, depending on coordinate system)
                    let py = map_height
                        - 1
                        - ((chunk_y + num_chunks_y) as u32 * config.chunk_height + tile_y);

                    // Put a single pixel for this tile
                    map_image.put_pixel(px, py, color);
                }
            }
        }
    }

    map_image
        .save("world_map.png")
        .expect("Failed to save image");
    info!("World image generated and saved as world_map.png");
}

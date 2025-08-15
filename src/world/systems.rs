use crate::prelude::*;
use crate::world::*;

// #[derive(Resource, Default)]
// #[insert_resource(plugin = WorldPlugin)]
// pub struct ChunkManager(pub HashMap<(i32, i32), Entity>);

// fn world_pos_to_chunk_coords(pos: Vec2, config: &WorldGeneratationConfig) -> (i32, i32) {
//     let chunk_width_world = config.chunk_width as f32 * config.tile_size;
//     let chunk_height_world = config.chunk_height as f32 * config.tile_size;

//     let chunk_x = (pos.x / chunk_width_world).floor() as i32;
//     let chunk_y = (pos.y / chunk_height_world).floor() as i32;
//     (chunk_x, chunk_y)
// }

// #[add_system(
//     schedule = Update,
//     plugin = WorldPlugin,
//     run_if = on_timer(Duration::from_millis(500)),
//     run_if = in_state(GameState::InGame)
// )]
// fn load_near_chunks(
//     mut commands: Commands,
//     player_query: Query<&Transform, With<Player>>,
//     mut chunks: ResMut<ChunkManager>,
//     generator: Res<WorldGenerator>,
//     config: Res<WorldGeneratationConfig>,
//     asset_server: Res<AssetServer>,
// ) {
//     let player_transform = match player_query.single() {
//         Ok(t) => t,
//         Err(_) => return,
//     };

//     let (player_chunk_x, player_chunk_y) =
//         world_pos_to_chunk_coords(player_transform.translation.truncate(), &config);

//     let tilemap_image = asset_server.load("textures/tiles/tilemap.png");

//     for dy in -config.load_radius..=config.load_radius {
//         for dx in -config.load_radius..=config.load_radius {
//             let chunk_coord = (player_chunk_x + dx, player_chunk_y + dy);

//             if chunks.0.contains_key(&chunk_coord) {
//                 continue;
//             }

//             let chunk_entity = generator.generate_chunk(
//                 &mut commands,
//                 chunk_coord.0,
//                 chunk_coord.1,
//                 &config,
//                 tilemap_image.clone(),
//             );

//             chunks.0.insert(chunk_coord, chunk_entity);
//             info!(" - Generated chunk at {:?}", chunk_coord);
//         }
//     }
// }

// #[add_system(
//     schedule = Update,
//     plugin = WorldPlugin,
//     run_if = on_timer(Duration::from_millis(500)),
//     run_if = in_state(GameState::InGame),
//     after = load_near_chunks)
// ]
// fn unload_far_chunks(
//     mut commands: Commands,
//     player_query: Query<&Transform, With<Player>>,
//     mut chunks: ResMut<ChunkManager>,
//     config: Res<WorldGeneratationConfig>,
// ) {
//     let player_transform = match player_query.single() {
//         Ok(t) => t,
//         Err(_) => return,
//     };

//     let (player_chunk_x, player_chunk_y) =
//         world_pos_to_chunk_coords(player_transform.translation.truncate(), &config);

//     let unload_distance = config.load_radius * 2;

//     // Find chunks to unload (too far away)
//     let to_unload: Vec<(i32, i32)> = chunks
//         .0
//         .iter()
//         .filter(|&(&chunk_coord, _)| {
//             let (cx, cy) = chunk_coord;
//             (cx - player_chunk_x).abs() > unload_distance
//                 || (cy - player_chunk_y).abs() > unload_distance
//         })
//         .map(|(&chunk_coord, _)| chunk_coord)
//         .collect();

//     for chunk_coord in to_unload {
//         if let Some(entity) = chunks.0.remove(&chunk_coord) {
//             commands.entity(entity).despawn();
//             info!(" - Unloaded chunk at {:?}", chunk_coord);
//         }
//     }
// }

// #[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::InGame))]
// fn draw_chunk_grid(
//     mut gizmos: Gizmos,
//     player_query: Query<&Transform, With<Player>>,
//     config: Res<WorldGeneratationConfig>,
// ) {
//     let Ok(player_transform) = player_query.single() else {
//         return;
//     };
//     let player_pos = player_transform.translation.truncate();
//     let (player_chunk_x, player_chunk_y) = world_pos_to_chunk_coords(player_pos, &config);

//     let chunk_width_world = config.chunk_width as f32 * config.tile_size;
//     let chunk_height_world = config.chunk_height as f32 * config.tile_size;

//     let draw_radius = config.load_radius + 1;

//     let min_chunk_x = player_chunk_x - draw_radius;
//     let max_chunk_x = player_chunk_x + draw_radius;
//     let min_chunk_y = player_chunk_y - draw_radius;
//     let max_chunk_y = player_chunk_y + draw_radius;

//     let offset = -config.tile_size / 2.0;

//     let min_x = min_chunk_x as f32 * chunk_width_world + offset;
//     let max_x = (max_chunk_x + 1) as f32 * chunk_width_world + offset;
//     let min_y = min_chunk_y as f32 * chunk_height_world + offset;
//     let max_y = (max_chunk_y + 1) as f32 * chunk_height_world + offset;

//     for chunk_x in min_chunk_x..=max_chunk_x + 1 {
//         let world_x = chunk_x as f32 * chunk_width_world + offset;
//         gizmos.line(
//             Vec3::new(world_x, min_y, 100.0),
//             Vec3::new(world_x, max_y, 100.0),
//             Color::srgb(1.0, 1.0, 0.0),
//         );
//     }

//     // Horizontal lines
//     for chunk_y in min_chunk_y..=max_chunk_y + 1 {
//         let world_y = chunk_y as f32 * chunk_height_world + offset;
//         gizmos.line(
//             Vec3::new(min_x, world_y, 100.0),
//             Vec3::new(max_x, world_y, 100.0),
//             Color::srgb(1.0, 1.0, 0.0),
//         );
//     }

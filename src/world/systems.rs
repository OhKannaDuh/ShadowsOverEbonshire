use std::time::Duration;

use bevy::gizmos::config;
use bevy::platform::collections::HashSet;
use bevy::time::common_conditions::on_timer;
use noise::{NoiseFn, Perlin};

use bevy_simple_tilemap::*;

use crate::player::Player;
use crate::prelude::*;
use crate::world::*;
use bevy::render::camera::ScalingMode;

#[derive(Resource, Default)]
#[insert_resource(plugin = WorldPlugin)]
struct GeneratedChunks(HashSet<(i32, i32)>);

#[add_system(schedule = OnEnter(GameState::InGame), plugin = WorldPlugin)]
fn setup_world(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("textures/tiles/tilemap.png");
    let atlas = TextureAtlasLayout::from_grid(uvec2(32, 32), 10, 10, None, None);
    let atlas_handle = texture_atlases.add(atlas);

    commands.spawn((
        TileMap::new(image, atlas_handle),
        Transform::from_translation(Vec3::new(0.0, 0.0, -64.0)),
    ));
}

fn world_pos_to_chunk_coords(pos: Vec2, config: &Res<WorldGeneratationConfig>) -> (i32, i32) {
    let chunk_width_world = config.chunk_width as f32 * config.tile_size;
    let chunk_height_world = config.chunk_height as f32 * config.tile_size;

    let chunk_x = (pos.x / chunk_width_world).floor() as i32;
    let chunk_y = (pos.y / chunk_height_world).floor() as i32;
    (chunk_x, chunk_y)
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = on_timer(Duration::from_millis(200)), run_if = in_state(GameState::InGame))]
fn load_near_chunks(
    mut tilemap_query: Query<&mut TileMap>,
    player_query: Query<&Transform, With<Player>>,
    mut chunks: ResMut<GeneratedChunks>,
    generator: Res<WorldGenerator>,
    config: Res<WorldGeneratationConfig>,
) {
    let player_transform = match player_query.single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let mut tilemap = match tilemap_query.single_mut() {
        Ok(tilemap) => tilemap,
        Err(_) => return,
    };

    let player_pos = player_transform.translation.truncate();
    let (player_chunk_x, player_chunk_y) = world_pos_to_chunk_coords(player_pos, &config);

    let mut generated_tiles: Vec<(IVec3, Option<Tile>)> = Vec::new();

    for dy in -config.load_radius..=config.load_radius {
        for dx in -config.load_radius..=config.load_radius {
            let chunk_coord = (player_chunk_x + dx, player_chunk_y + dy);

            if chunks.0.contains(&chunk_coord) {
                continue;
            }

            generated_tiles.extend(generator.generate_chunk(chunk_coord.0, chunk_coord.1, &config));
            chunks.0.insert(chunk_coord);
            info!(" - Generated chunk at {:?}", chunk_coord);
        }
    }

    if !generated_tiles.is_empty() {
        tilemap.set_tiles(generated_tiles);
        info!("   - Loaded chunks: {}", chunks.0.len());
    }
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = on_timer(Duration::from_millis(500)), run_if = in_state(GameState::InGame), after = load_near_chunks)]
fn unload_far_chunks(
    mut tilemap_query: Query<&mut TileMap>,
    player_query: Query<&Transform, With<Player>>,
    mut chunks: ResMut<GeneratedChunks>,
    config: Res<WorldGeneratationConfig>,
) {
    let player_transform = match player_query.single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let mut tilemap = match tilemap_query.single_mut() {
        Ok(tilemap) => tilemap,
        Err(_) => return,
    };

    let player_pos = player_transform.translation.truncate();
    let (player_chunk_x, player_chunk_y) = world_pos_to_chunk_coords(player_pos, &config);

    let unload_distance = config.load_radius * 2;

    // Collect chunks to unload
    let to_unload: Vec<(i32, i32)> = chunks
        .0
        .iter()
        .copied()
        .filter(|&(cx, cy)| {
            (cx - player_chunk_x).abs() > unload_distance
                || (cy - player_chunk_y).abs() > unload_distance
        })
        .collect();

    if to_unload.is_empty() {
        return;
    }

    // Clear tiles in the tilemap for those chunks
    let mut clear_tiles: Vec<(IVec3, Option<Tile>)> = Vec::new();

    for &(cx, cy) in &to_unload {
        let offset_x = cx * config.chunk_width_i32;
        let offset_y = cy * config.chunk_height_i32;

        for y in 0..config.chunk_height {
            for x in 0..config.chunk_width {
                clear_tiles.push((ivec3(offset_x + x as i32, offset_y + y as i32, 0), None));
            }
        }

        // Remove from your generated chunk tracking
        chunks.0.remove(&(cx, cy));

        // Remove from the tilemap's chunk storage
        tilemap.chunks.remove(&IVec3::new(cx, cy, 0));

        info!(" - Unloaded chunk at ({}, {})", cx, cy);
    }

    if !clear_tiles.is_empty() {
        tilemap.set_tiles(clear_tiles);
        info!("   - Loaded chunks remaining: {}", chunks.0.len());
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

    // Compute chunk bounds
    let min_chunk_x = player_chunk_x - draw_radius;
    let max_chunk_x = player_chunk_x + draw_radius;
    let min_chunk_y = player_chunk_y - draw_radius;
    let max_chunk_y = player_chunk_y + draw_radius;

    // Offset so lines align with tile edges instead of centers
    let offset = -config.tile_size / 2.0;

    let min_x = min_chunk_x as f32 * chunk_width_world + offset;
    let max_x = (max_chunk_x + 1) as f32 * chunk_width_world + offset;
    let min_y = min_chunk_y as f32 * chunk_height_world + offset;
    let max_y = (max_chunk_y + 1) as f32 * chunk_height_world + offset;

    // Vertical lines
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

#[allow(dead_code)]
enum Anchor {
    BottomLeft,
    Center,
}

#[allow(dead_code)]
struct Rect {
    anchor: Anchor,
    position: Vec2,
    size: Vec2,
}

impl Rect {
    #[inline]
    fn is_intersecting(&self, other: &Rect) -> bool {
        self.get_center_position()
            .distance(other.get_center_position())
            < (self.get_radius() + other.get_radius())
    }

    #[inline]
    fn get_center_position(&self) -> Vec2 {
        match self.anchor {
            Anchor::BottomLeft => self.position + (self.size / 2.0),
            Anchor::Center => self.position,
        }
    }

    #[inline]
    fn get_radius(&self) -> f32 {
        let half_size = self.size / 2.0;
        (half_size.x * half_size.x + half_size.y * half_size.y).sqrt()
    }
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::InGame))]
pub fn draw_camera_rects(
    mut gizmos: Gizmos,
    window_query: Query<&Window>,
    camera_query: Query<(&GlobalTransform, &Projection), With<Camera2d>>,
) {
    let window = match window_query.iter().next() {
        Some(win) => win,
        None => return,
    };

    let window_width = window.width();
    let window_height = window.height();
    let window_aspect = window_width / window_height;

    for (transform, projection) in camera_query.iter() {
        // Camera position in world space
        let camera_pos = transform.translation();

        // if projection is Orthographic
        ///pub enum Projection {
        //     Perspective(PerspectiveProjection),
        //     Orthographic(OrthographicProjection),
        //     Custom(CustomProjection),
        // }
        if let Projection::Orthographic(ortho) = projection {
            let viewport_height = match ortho.scaling_mode {
                ScalingMode::FixedVertical { viewport_height } => viewport_height,
                ScalingMode::FixedHorizontal { viewport_width } => viewport_width / window_aspect,
                ScalingMode::WindowSize => window_height,
                _ => 0.0,
            };

            let viewport_width = viewport_height * window_aspect;

            let half_size = Vec2::new(viewport_width / 2.0, viewport_height / 2.0);
            let center = camera_pos.truncate();

            // Corners of the camera rect in world space (z=100 for gizmos)
            let bl = Vec3::new(center.x - half_size.x, center.y - half_size.y, 100.0);
            let br = Vec3::new(center.x + half_size.x, center.y - half_size.y, 100.0);
            let tr = Vec3::new(center.x + half_size.x, center.y + half_size.y, 100.0);
            let tl = Vec3::new(center.x - half_size.x, center.y + half_size.y, 100.0);

            let color = Color::srgb(1.0, 1.0, 1.0);

            gizmos.line(bl, br, color);
            gizmos.line(br, tr, color);
            gizmos.line(tr, tl, color);
            gizmos.line(tl, bl, color);
        }
    }
}

use bevy::math::DVec2;
use fast_poisson::Poisson2D;

use crate::prelude::*;
use crate::world::*;

const TILE_SIZE_PX: f32 = 32.0;
const CHUNK_TILES: f32 = 64.0;
const WORLD_CHUNKS_X: f32 = 48.0;
const WORLD_CHUNKS_Y: f32 = 48.0;

fn poisson_radius_for_count(area: f32, target: f32) -> f32 {
    ((1.154_700_5 * area) / target).sqrt()
}

#[derive(Reflect, Debug)]
enum WorldFeatureType {
    Core,
    Landmark,
}

#[derive(Reflect, Debug)]
struct WorldFeature {
    feature_type: WorldFeatureType,
    position: Vec3,
    radius: f32,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
#[insert_resource(plugin = WorldPlugin)]
struct WorldFeatures {
    features: Vec<WorldFeature>,
}

#[add_system(
    schedule = OnEnter(GameState::GeneratingMap),
    plugin = WorldPlugin,
)]
fn add_core_feature(mut world_features: ResMut<WorldFeatures>) {
    world_features.features.push(WorldFeature {
        feature_type: WorldFeatureType::Core,
        position: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.5 * CHUNK_TILES,
    });
}

#[add_system(
    schedule = OnEnter(GameState::GeneratingMap),
    plugin = WorldPlugin,
    after = add_core_feature
)]
fn sample_world_features(
    mut world_features: ResMut<WorldFeatures>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // let seed: u64 = 0xCAFEF00D;
    let seed: u64 = 23534536336534;

    let world_tiles_x = WORLD_CHUNKS_X * CHUNK_TILES;
    let world_tiles_y = WORLD_CHUNKS_Y * CHUNK_TILES;
    let area = world_tiles_x * world_tiles_y;

    let target_sites = 28.0;
    let mut r_tiles = poisson_radius_for_count(area, target_sites);
    r_tiles = r_tiles.clamp(140.0, 180.0);

    let dims = [world_tiles_x, world_tiles_y];
    let half = Vec2::new(dims[0] * 0.5, dims[1] * 0.5);

    let points: Vec<Vec2> = Poisson2D::new()
        .with_seed(seed)
        .with_dimensions([dims[0], dims[1]], r_tiles)
        .iter()
        .map(|[x, y]| Vec2::new(x, y) - half)
        .collect();

    for point in points {
        world_features.features.push(WorldFeature {
            feature_type: WorldFeatureType::Landmark,
            position: point.extend(0.0),
            radius: r_tiles,
        });
    }

    next_state.set(GameState::InGame);
}

fn world_tile_pos_to_chunk_pos(tile_pos: Vec2) -> Vec2 {
    Vec2::new(
        (tile_pos.x / CHUNK_TILES).floor(),
        (tile_pos.y / CHUNK_TILES).floor(),
    )
}

#[add_system(
    schedule = Update,
    plugin = WorldPlugin,
    run_if = in_state(GameState::InGame),
)]
fn render_world_features(world_features: Res<WorldFeatures>, mut gizmos: Gizmos) {
    for feature in world_features.features.iter() {
        match feature.feature_type {
            WorldFeatureType::Core => {
                gizmos.circle_2d(
                    feature.position.truncate() * TILE_SIZE_PX,
                    feature.radius,
                    Color::srgb(0.12, 0.56, 1.0),
                );
            }
            WorldFeatureType::Landmark => {
                gizmos.circle_2d(
                    feature.position.truncate() * TILE_SIZE_PX,
                    feature.radius,
                    Color::srgb(0.4, 0.2, 0.6),
                );

                let chunk_pos = world_tile_pos_to_chunk_pos(feature.position.truncate());
                let chunk_world_pos = chunk_pos * CHUNK_TILES * TILE_SIZE_PX;
                let chunk_center_world_pos =
                    chunk_world_pos + Vec2::splat(0.5 * CHUNK_TILES * TILE_SIZE_PX);
                gizmos.rect_2d(
                    chunk_center_world_pos,
                    Vec2::splat(CHUNK_TILES * TILE_SIZE_PX),
                    Color::srgb(1.0, 0.8, 0.2),
                );
            }
        }
    }
}

#[add_system(
    schedule = Update,
    plugin = WorldPlugin,
    run_if = in_state(GameState::InGame),
)]
fn render_chunk_boundaries(mut gizmos: Gizmos) {
    let cell_count = UVec2::new(WORLD_CHUNKS_X as u32, WORLD_CHUNKS_Y as u32);
    let spacing = Vec2::splat(CHUNK_TILES * TILE_SIZE_PX);
    gizmos.grid_2d(
        Vec2::splat(0.0),
        cell_count,
        spacing,
        Color::srgb(0.7, 0.7, 0.7),
    );
}

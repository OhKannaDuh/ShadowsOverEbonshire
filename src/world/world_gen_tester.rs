use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use image::Rgba;

use crate::prelude::*;
use crate::world::systems::*;
use crate::world::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub texture: Handle<Image>,
}

fn spiral_offsets(radius: i32) -> impl Iterator<Item = (i32, i32)> {
    let mut out = Vec::with_capacity(((radius * 2 + 1).pow(2)) as usize);

    out.push((0, 0));
    for r in 1..=radius {
        let x_min = -r;
        let x_max = r;
        let y_min = -r;
        let y_max = r;

        // bottom edge: left → right
        for x in x_min..=x_max {
            out.push((x, y_min));
        }
        // right edge: bottom+1 → top
        for y in (y_min + 1)..=y_max {
            out.push((x_max, y));
        }
        // top edge: right-1 → left
        for x in (x_min..x_max).rev() {
            out.push((x, y_max));
        }
        // left edge: top-1 → bottom+1
        for y in (y_min + 1..y_max).rev() {
            out.push((x_min, y));
        }
    }

    out.into_iter()
}

const MAX_CHUNKS_PER_FRAME: u32 = 16;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::InGame))]
fn generate_map(
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    config: Res<WorldGeneratationConfig>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let Ok(cam_tf) = camera_query.single() else {
        return;
    };

    let cam_chunk = (
        (cam_tf.translation.x / config.chunk_width as f32).floor() as i32,
        (cam_tf.translation.y / config.chunk_height as f32).floor() as i32,
    );

    let mut generated = 0;
    for (dx, dy) in spiral_offsets(config.load_radius) {
        let chunk_x = cam_chunk.0 + dx;
        let chunk_y = cam_chunk.1 + dy;

        if !chunks.0.contains_key(&(chunk_x, chunk_y)) {
            generate_blank_chunk(
                chunk_x,
                chunk_y,
                &mut chunks,
                &mut commands,
                &mut images,
                &config,
            );
            generated += 1;
            if generated >= MAX_CHUNKS_PER_FRAME {
                break;
            }
        }
    }
}

fn generate_blank_chunk(
    x: i32,
    y: i32,
    chunks: &mut ResMut<ChunkManager>,
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    config: &Res<WorldGeneratationConfig>,
) {
    let image = Image::new_fill(
        Extent3d {
            width: config.chunk_width,
            height: config.chunk_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let handle = images.add(image);
    let handle_clone = handle.clone();

    chunks.0.insert(
        (x, y),
        commands
            .spawn((
                Chunk {
                    x,
                    y,
                    texture: handle,
                },
                Transform::from_translation(Vec3::new(
                    x as f32 * config.chunk_width as f32,
                    y as f32 * config.chunk_height as f32,
                    0.0,
                )),
                Sprite {
                    image: handle_clone,
                    color: Color::WHITE,
                    ..default()
                },
            ))
            .id(),
    );
}

#[add_system(schedule = Update, plugin = WorldPlugin, after = generate_map)]
fn fill_new_chunks(
    mut images: ResMut<Assets<Image>>,
    config: Res<WorldGeneratationConfig>,
    generator: Res<WorldGenerator>,
    mut q_new: Query<&Chunk, Added<Chunk>>,
) {
    for chunk in &mut q_new {
        let Some(img) = images.get_mut(&chunk.texture) else {
            continue;
        };
        let data = img.data.as_mut().expect("Image data should be initialized");

        if data.len() != (config.chunk_width * config.chunk_height * 4) as usize {
            data.resize((config.chunk_width * config.chunk_height * 4) as usize, 0);
        }

        // World-space origin of this chunk
        let base_x = chunk.x * config.chunk_width as i32;
        let base_y = chunk.y * config.chunk_height as i32;

        let row_stride = (config.chunk_width * 4) as usize;

        for y in 0..config.chunk_height {
            let inv_y = config.chunk_height - 1 - y;
            let row_start = inv_y as usize * row_stride;
            let row = &mut data[row_start..row_start + row_stride];

            for x in 0..config.chunk_width {
                let wx = base_x + x as i32;
                let wy = base_y + y as i32;

                let p = generator.get_point(wx, wy);
                let c = tile_id_to_color(p.tile_id); // Rgba<u8>

                let i = x as usize * 4;
                row[i] = c.0[0];
                row[i + 1] = c.0[1];
                row[i + 2] = c.0[2];
                row[i + 3] = c.0[3];
            }
        }
    }
}

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

        TileId::Snow => Rgba([255, 250, 250, 255]), // Snow white

        TileId::Beach => Rgba([255, 228, 196, 255]), // Bisque
        TileId::ShallowOcean => Rgba([70, 130, 180, 255]), // Steel blue
        TileId::Ocean => Rgba([0, 0, 139, 255]),     // Dark blue
        TileId::DeepOcean => Rgba([0, 0, 255, 255]), // Blue
    }
}

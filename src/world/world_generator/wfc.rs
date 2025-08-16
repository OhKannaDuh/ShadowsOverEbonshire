use crate::prelude::*;
use crate::world::world_generator::*;
use bevy::{
    tasks::{AsyncComputeTaskPool, Task},
    time::Stopwatch,
};
use image::{Rgba, RgbaImage};
use wave::{WaveFunction, WaveTiles};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SocketType {
    Grass,
    Rose,
    Dandelion,
    Tree,
}

#[derive(WaveTiles, Copy, Clone, Debug, Eq, PartialEq)]
#[wave_socket(SocketType)]
pub enum Slate {
    #[socket_vertical  ([(SocketType::Grass, 95u32), (SocketType::Rose, 2u32), (SocketType::Dandelion, 3u32)])]
    #[socket_horizontal([(SocketType::Grass, 95u32), (SocketType::Rose, 2u32), (SocketType::Dandelion, 3u32)])]
    Grass,

    #[socket_vertical  ([(SocketType::Rose, 2u32)])]
    #[socket_horizontal([(SocketType::Rose, 2u32)])]
    Rose,

    #[socket_vertical  ([(SocketType::Dandelion, 5u32)])]
    #[socket_horizontal([(SocketType::Dandelion, 5u32)])]
    Dandelion,

    #[socket_vertical  ([(SocketType::Tree, 5u32), (SocketType::Grass, 2u32)])]
    #[socket_horizontal([(SocketType::Tree, 5u32), (SocketType::Grass, 2u32)])]
    Tree,
}

impl Slate {
    fn get_color(&self) -> Rgba<u8> {
        match self {
            Slate::Grass => Rgba([34, 139, 34, 255]),
            Slate::Rose => Rgba([220, 20, 60, 255]),
            Slate::Dandelion => Rgba([255, 215, 0, 255]),
            Slate::Tree => Rgba([0, 100, 0, 255]),
        }
    }
}

const WORLD_WIDTH: f32 = 48.0; // (chunks)
const WORLD_HEIGHT: f32 = 48.0; // (chunks)

const CHUNK_DIMENSION: f32 = 64.0; // (tiles)
const SLATE_DIMENSION: f32 = 8.0; // (tiles)
const TILE_DIMENSION: f32 = 32.0; // (pixels)

const WORLD_WIDTH_PX: f32 = WORLD_WIDTH * CHUNK_DIMENSION * TILE_DIMENSION;
const WORLD_HEIGHT_PX: f32 = WORLD_HEIGHT * CHUNK_DIMENSION * TILE_DIMENSION;
const SLATE_DIMENSION_PX: f32 = SLATE_DIMENSION * TILE_DIMENSION;

const WORLD_WIDTH_SLATES_F: f32 = WORLD_WIDTH_PX / SLATE_DIMENSION_PX;
const WORLD_HEIGHT_SLATES_F: f32 = WORLD_HEIGHT_PX / SLATE_DIMENSION_PX;

// If you prefer const ints:
const WORLD_WIDTH_SLATES: usize = WORLD_WIDTH_SLATES_F as usize;
const WORLD_HEIGHT_SLATES: usize = WORLD_HEIGHT_SLATES_F as usize;

#[derive(Resource, Default)]
#[insert_resource(plugin = WorldPlugin)]
pub struct WfcTask {
    pub task: Option<Task<Result<Vec<((usize, usize), Slate)>, String>>>,
    pub timer: Stopwatch,
}

#[add_system(
    schedule = OnEnter(GameState::GeneratingMap),
    plugin = WorldPlugin,
    after = sample_world_features
)]
fn wave_function_collapse(mut wfc: ResMut<WfcTask>) {
    if wfc.task.is_some() {
        return;
    }

    let seed: u64 = 23_534_536_336_534;
    let w = WORLD_WIDTH_SLATES;
    let h = WORLD_HEIGHT_SLATES;

    wfc.timer = Stopwatch::new();
    wfc.task = Some(AsyncComputeTaskPool::get().spawn(async move {
        let mut wave = WaveFunction::<Slate>::new([w, h], seed);

        let (cx, cy) = (w / 2, h / 2);
        wave.set((cx, cy), Slate::Grass);

        wave.collapse::<SocketType>()
            .map_err(|e| format!("Wave contradiction at {:?}", e.at))
    }));
}

#[add_system(
    schedule = Update,
    plugin = WorldPlugin,
    run_if = in_state(GameState::GeneratingMap)
)]
fn poll_wave_function_collapse(
    mut wfc: ResMut<WfcTask>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    wfc.timer.tick(time.delta());

    use futures_lite::future;
    if let Some(task) = wfc.task.as_mut() {
        if let Some(result) = future::block_on(future::poll_once(task)) {
            wfc.task = None;

            info!(
                "Wave function collapse completed in {:?}",
                wfc.timer.elapsed()
            );

            match result {
                Ok(tiles) => {
                    const SCALE: u32 = 1;
                    let width_px = (WORLD_WIDTH_SLATES as u32) * SCALE;
                    let height_px = (WORLD_HEIGHT_SLATES as u32) * SCALE;

                    let mut img = RgbaImage::from_pixel(width_px, height_px, Rgba([0, 0, 0, 255]));

                    for ((x, y), slate) in tiles {
                        let color = slate.get_color();
                        let px = (x as u32) * SCALE;
                        let py = (y as u32) * SCALE;
                        for dy in 0..SCALE {
                            for dx in 0..SCALE {
                                img.put_pixel(px + dx, py + dy, color);
                            }
                        }
                    }

                    img.save("wfc_generation.png")
                        .expect("Failed to save slate image");

                    next_state.set(GameState::InGame);
                }
                Err(err) => {
                    error!("WFC failed: {err}");
                }
            }
        }
    }
}

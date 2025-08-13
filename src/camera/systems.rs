use bevy::render::camera::ScalingMode;

use crate::camera::*;

#[add_system(schedule = OnEnter(GameState::InGame), plugin = CameraPlugin)]
fn spawn_camera(mut commands: Commands) {
    info!("Spawning main camera");
    commands.spawn((
        MainCamera {
            base_speed: 5.0,
            max_speed: 10.0,
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 2160.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// #[add_system(schedule = Update, plugin = CameraPlugin, run_if = in_state(GameState::InGame))]
fn update_camera(
    mut camera_query: Query<(&mut Transform, &mut Projection, &MainCamera), Without<CameraFocus>>,
    focus_query: Query<(&Transform, &CameraFocus), With<CameraFocus>>,
    time: Res<Time>,
) {
    if let Ok((mut camera_transform, mut projection, camera)) = camera_query.single_mut() {
        // Collect all active foci
        let mut sum = Vec3::ZERO;
        let mut count = 0;

        // Track bounds
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for (transform, focus) in focus_query.iter() {
            if focus.0 {
                let pos = transform.translation;
                sum += pos;
                count += 1;

                min_x = min_x.min(pos.x);
                max_x = max_x.max(pos.x);
                min_y = min_y.min(pos.y);
                max_y = max_y.max(pos.y);
            }
        }

        if count == 0 {
            return;
        }

        let avg_pos = sum / count as f32;

        let target = Vec3::new(avg_pos.x, avg_pos.y, camera_transform.translation.z);
        let current = camera_transform.translation;
        let distance = current.distance(target);

        let speed = camera
            .base_speed
            .max((distance / 10.0) * camera.max_speed)
            .min(camera.max_speed);

        camera_transform.translation =
            current.lerp(target, 1.0 - (-speed * time.delta_secs()).exp());

        if let Projection::Orthographic(ref mut ortho) = *projection {
            if let ScalingMode::FixedVertical {
                ref mut viewport_height,
            } = ortho.scaling_mode
            {
                let width = max_x - min_x;
                let height = max_y - min_y;

                let margin = 1.2;
                let desired_width = width * margin;
                let desired_height = height * margin;

                let area_width = ortho.area.max.x - ortho.area.min.x;
                let area_height = ortho.area.max.y - ortho.area.min.y;
                let aspect_ratio = if area_height != 0.0 {
                    area_width / area_height
                } else {
                    16.0 / 9.0
                };

                let mut new_height = desired_width / aspect_ratio;
                new_height = new_height.max(desired_height);
                new_height = new_height.max(2160.0);

                *viewport_height = new_height;
            }
        }
    }
}

use crate::camera::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

// Tunables
const PAN_SPEED: f32 = 1200.0; // world units per second at 1x
const PAN_SPEED_FAST: f32 = 2400.0; // when Shift is held
const ZOOM_LINE_STEP: f32 = 1.12; // multiplicative zoom per wheel line "tick"
const ZOOM_PIXEL_STEP: f32 = 1.002; // multiplicative zoom per wheel pixel
const MIN_VIEWPORT_HEIGHT: f32 = 64.0;
const MAX_VIEWPORT_HEIGHT: f32 = 16384.0;

const MIN_SCALE: f32 = 0.05; // smaller = more zoomed in
const MAX_SCALE: f32 = 50.0;

#[add_system(
    schedule = Update,
    plugin = CameraPlugin,
    run_if = in_state(GameState::InGame)
)]
fn camera_pan_zoom(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: EventReader<MouseWheel>,
    mut q_cam: Query<(&mut Transform, &mut Projection, &MainCamera)>,
) {
    let Ok((mut tf, mut proj, _mc)) = q_cam.get_single_mut() else {
        return;
    };

    // --------- PAN (WASD) ----------
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }

    if dir.length_squared() > 0.0 {
        dir = dir.normalize();
        // Move faster with Shift
        let base = if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            PAN_SPEED_FAST
        } else {
            PAN_SPEED
        };

        // Keep perceived speed roughly constant across zoom levels by scaling with current viewport height
        let mut scale_for_speed = 1.0;
        if let Projection::Orthographic(ref ortho) = *proj {
            // viewport_height is "world units visible vertically"; bigger = more zoomed out
            // Using a fraction of it keeps panning feel similar regardless of zoom level
            scale_for_speed = (ortho.area.max.y - ortho.area.min.y) / 1080.0; // 1080 is arbitrary reference
        }

        let delta = dir * base * scale_for_speed * time.delta_secs();
        tf.translation.x += delta.x;
        tf.translation.y += delta.y;
    }

    // --------- ZOOM (Mouse Wheel) ----------
    let ortho = match *proj {
        Projection::Orthographic(ref mut o) => o,
        _ => return,
    };

    let mut zoom_mul = 1.0_f32;
    for ev in wheel.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    zoom_mul /= ZOOM_LINE_STEP.powf(ev.y as f32);
                }
                if ev.y < 0.0 {
                    zoom_mul *= ZOOM_LINE_STEP.powf(-ev.y as f32);
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.y > 0.0 {
                    zoom_mul /= ZOOM_PIXEL_STEP.powf(ev.y as f32);
                }
                if ev.y < 0.0 {
                    zoom_mul *= ZOOM_PIXEL_STEP.powf(-ev.y as f32);
                }
            }
        }
    }
    if zoom_mul != 1.0 {
        ortho.scale = (ortho.scale * zoom_mul).clamp(MIN_SCALE, MAX_SCALE);
    }
}

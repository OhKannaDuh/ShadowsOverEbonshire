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
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[add_system(schedule = Update, plugin = CameraPlugin, run_if = in_state(GameState::InGame))]
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
                new_height = new_height.max(1080.0);

                *viewport_height = new_height;
            }
        }
    }
}

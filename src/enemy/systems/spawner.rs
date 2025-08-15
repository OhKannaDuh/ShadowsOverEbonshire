use rand::Rng;

use crate::actor::*;
use crate::animated_sprite::*;
use crate::camera::MainCamera;
use crate::enemy::*;

#[derive(Resource)]
#[insert_resource(plugin = EnemyPlugin)]
struct EnemySpawnTimer {
    pub timer: Timer,
    pub amount: u32,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        let secs = rand::thread_rng().gen_range(5.0..=10.0);
        debug!("Next enemy spawn in: {:.2} seconds", secs);
        EnemySpawnTimer {
            timer: Timer::from_seconds(secs, TimerMode::Repeating),
            amount: rand::thread_rng().gen_range(5..=10),
        }
    }
}

fn random_point_around_camera(camera_pos: &Vec3, min_radius: f32, max_radius: f32) -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(min_radius..max_radius);
    let x = camera_pos.x + radius * angle.cos();
    let y = camera_pos.y + radius * angle.sin();
    Vec3::new(x, y, 0.0)
}

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    enemies_query: Query<&Enemy>,
    camera_query: Query<(&Transform, &Projection), With<MainCamera>>,
    assets: Res<SlimeAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    let (camera_transform, projection) = if let Ok(data) = camera_query.single() {
        data
    } else {
        debug!("No camera found, skipping enemy spawn");
        return;
    };

    let (min_radius, max_radius) = if let Projection::Orthographic(ortho) = projection {
        let width = ortho.area.max.x - ortho.area.min.x;
        let height = ortho.area.max.y - ortho.area.min.y;

        let base_radius = width.max(height) * 0.8;

        (base_radius, base_radius * 1.2)
    } else {
        (500.0, 600.0)
    };

    if spawn_timer.timer.finished() {
        debug!("Spawning enemy");

        for _ in 0..spawn_timer.amount {
            let spawn_pos =
                random_point_around_camera(&camera_transform.translation, min_radius, max_radius);

            commands.spawn((
                Enemy,
                Name::new("Enemy"),
                TeamFlag(Team::Enemy),
                Sprite {
                    image: assets.sprite.clone(),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    texture_atlas: Some(TextureAtlas {
                        layout: assets.layout.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                AnimatedSprite {
                    animations: {
                        let mut map = HashMap::new();
                        map.insert(
                            "move".to_string(),
                            Animation {
                                frames: (0..5).collect(),
                                durations: vec![0.1; 6],
                            },
                        );
                        map
                    },
                    current_animation: "move".to_string(),
                    current_frame: 0,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                Speed(64.0),
                Aabb::from_min_max(Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 0.0)),
                Transform::from_translation(spawn_pos),
            ));
        }

        info!(
            "Current Enemy Count: {}",
            enemies_query.iter().count() + spawn_timer.amount as usize
        );

        let new_secs = rand::thread_rng().gen_range(5.0..=10.0);
        debug!("Next enemy spawn in: {:.2} seconds", new_secs);
        spawn_timer
            .timer
            .set_duration(std::time::Duration::from_secs_f32(new_secs));

        spawn_timer.amount = rand::thread_rng().gen_range(900..=1000);

        spawn_timer.timer.reset();
    }
}

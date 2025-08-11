use crate::actor::Health;
use crate::prelude::*;

use crate::actor::Team;
use crate::actor::TeamFlag;
use crate::input::Action;
use crate::player::Player;
use crate::player::PlayerPlugin;

#[add_system(schedule = OnEnter(GameState::InGame), plugin = PlayerPlugin)]
fn spawn_player(mut commands: Commands) {
    info!("Spawning player");

    commands.spawn((
        Player,
        Name::new("Player"),
        TeamFlag(Team::Player),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Action::default_input_map(),
        // Aabb::from_min_max(Vec3::new(-16.0, -16.0, 0.0), Vec3::new(16.0, 16.0, 0.0)),
    ));
}

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::InGame))]
fn check_player_health(query: Query<&Health, With<Player>>) {
    for health in query.iter() {
        if health.current <= 0.0 {
            info!("Player health reached zero! Game Over!");
        }
    }
}

// @debug
// #[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::InGame))]
fn drain_health(mut query: Query<&mut Health, With<Player>>, time: Res<Time>) {
    let delta = time.delta_secs();

    for mut health in query.iter_mut() {
        health.current = (health.current - 10.0 * delta).max(0.0);
    }
}

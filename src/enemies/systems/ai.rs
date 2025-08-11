use crate::actor::*;
use crate::enemies::*;
use crate::player::*;

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn update_enemy_ai(
    mut query: Query<(&mut Transform, &EnemyAi, &Speed), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single().expect("No player found");

    for (mut transform, enemy_ai, speed) in query.iter_mut() {
        match enemy_ai.0 {
            EnemyAiType::Basic => {
                let direction = (player_transform.translation - transform.translation).truncate();
                if direction.length_squared() > 0.0 {
                    let dir_normalized = direction.normalize();
                    let speed = speed.0;

                    transform.translation.x += dir_normalized.x * speed * time.delta_secs();
                    transform.translation.y += dir_normalized.y * speed * time.delta_secs();
                }
            }
        }
    }
}

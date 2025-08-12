use crate::actor::*;
use crate::enemy::*;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn despawn_dead_enemies(query: Query<(Entity, &Health), With<Enemy>>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

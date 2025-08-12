use crate::actor::*;

#[add_system(schedule = Update, plugin = ActorPlugin, run_if = in_state(GameState::InGame))]
fn add_health_bar_to_actor(query: Query<Entity, Added<Actor>>, mut commands: Commands) {
    for entity in query.iter() {
        HealthBar::add_to_entity(entity, &mut commands);
    }
}

// #[add_system(schedule = Update, plugin = ActorPlugin, run_if = in_state(GameState::InGame))]
fn update_health_bars(
    actors: Query<(&Health, &Children)>,
    mut health_bar_query: Query<&mut Sprite, With<HealthBar>>,
) {
    for (health, children) in &actors {
        let health_percent = (health.current / health.max).clamp(0.0, 1.0);

        for child in children.iter() {
            if let Ok(mut sprite) = health_bar_query.get_mut(child) {
                // Adjust width based on health percent
                let max_width = 40.0;
                sprite.custom_size = Some(Vec2::new(max_width * health_percent, 6.0));
            }
        }
    }
}

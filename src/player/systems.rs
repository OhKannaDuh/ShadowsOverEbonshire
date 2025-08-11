use crate::input::Action;
use crate::prelude::*;

use crate::actor::Actor;
use crate::player::Player;
use crate::player::PlayerPlugin;

#[add_system(schedule = Startup, plugin = PlayerPlugin)]
fn spawn_player(mut commands: Commands) {
    info!("Spawning player");

    commands.spawn((
        Player,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Action::default_input_map(),
    ));

    // Needs sprite setting
}

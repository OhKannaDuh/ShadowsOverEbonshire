use crate::prelude::*;

use crate::actor::Speed;
use crate::input::Action;
use crate::input::InputPlugin;

#[add_system(schedule = Update, plugin = InputPlugin)]
fn handle_input(mut query: Query<(&mut Transform, &ActionState<Action>, &Speed)>, time: Res<Time>) {
    debug!("Handling player input");
    for (mut transform, action_state, speed) in query.iter_mut() {
        let mut movement = action_state.clamped_axis_pair(&Action::Move);
        movement *= time.delta_secs();
        movement *= speed.0;

        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

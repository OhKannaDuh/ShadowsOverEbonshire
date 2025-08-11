use crate::prelude::*;

use crate::camera::{CameraPlugin, MainCamera};

#[add_system(schedule = OnEnter(GameState::InGame), plugin = CameraPlugin)]
fn spawn_camera(mut commands: Commands) {
    info!("Spawning main camera");
    commands.spawn(MainCamera);
}

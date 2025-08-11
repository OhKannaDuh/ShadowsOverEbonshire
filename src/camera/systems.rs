use crate::prelude::*;

use crate::camera::{CameraPlugin, MainCamera};

#[add_system(schedule = Startup, plugin = CameraPlugin)]
fn spawn_camera(mut commands: Commands) {
    info!("Spawning main camera");
    commands.spawn(MainCamera);
}

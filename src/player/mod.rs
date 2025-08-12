use crate::actor::*;
use crate::camera::*;
use crate::input::*;
use crate::prelude::*;
use crate::weapons::components::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Experience(pub f32);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor, Experience, InputMap<Action>, EquippedWeapons, CameraFocus)]
pub struct Player;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct PlayerPlugin;

mod systems;

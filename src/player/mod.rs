use crate::prelude::*;

use crate::actor::Actor;
use crate::input::Action;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Experience(pub f32);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor, Experience, InputMap<Action>)]
pub struct Player;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct PlayerPlugin;

mod systems;

use crate::prelude::*;

use crate::actor::Actor;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Experience(f32);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor, Experience)]
struct Player;

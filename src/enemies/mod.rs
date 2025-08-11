use crate::prelude::*;

use crate::actor::Actor;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
struct Enemy;

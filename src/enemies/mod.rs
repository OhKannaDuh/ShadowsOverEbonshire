use crate::prelude::*;

use crate::actor::Actor;

mod contact_damage;
pub(crate) use contact_damage::*;

mod enemy_ai;
pub(crate) use enemy_ai::*;

mod velocity;
pub(crate) use velocity::*;

mod collision;
pub(crate) use collision::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor, ContactDamage, EnemyAi, Velocity)]
struct Enemy;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct EnemyPlugin;

mod systems;

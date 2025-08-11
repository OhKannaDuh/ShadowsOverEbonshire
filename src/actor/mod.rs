use crate::prelude::*;

use crate::entity::GameEntity;

mod speed;
pub(crate) use speed::*;

mod health;
pub(crate) use health::*;

mod team_flag;
pub(crate) use team_flag::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Speed, Health, TeamFlag, GameEntity)]
pub struct Actor;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct ActorPlugin;

mod systems;

use crate::prelude::*;

use crate::entity::GameEntity;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Reflect, Debug, Default)]
pub enum Team {
    #[default]
    Neutral,
    Player,
    Enemy,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct TeamFlag(pub Team);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Speed, Health, TeamFlag, GameEntity)]
pub struct Actor;

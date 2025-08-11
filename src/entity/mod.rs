use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(GlobalTransform, Transform, Sprite, Visibility)]
pub struct GameEntity;

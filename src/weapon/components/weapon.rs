use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct EquippedWeapons(pub Vec<Entity>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Weapon;

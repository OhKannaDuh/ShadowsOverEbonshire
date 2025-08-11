use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec3::ZERO)
    }
}

use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ContactDamage(pub f32);

impl Default for ContactDamage {
    fn default() -> Self {
        ContactDamage(1.0)
    }
}

use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(128.0)
    }
}

use crate::prelude::*;

#[butler_plugin]
#[add_plugin(to_group = EntityPlugins)]
pub struct WeaponPlugin;

pub mod components;
use crate::weapon::components::*;

mod systems;

use crate::prelude::*;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
pub struct WeaponPlugin;

pub mod components;
use crate::weapons::components::*;

mod systems;

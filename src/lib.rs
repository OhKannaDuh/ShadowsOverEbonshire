mod prelude;
use crate::prelude::*;

// Components
mod animated_sprite;

// Systems
mod camera;
mod input;

// Entities
mod actor;
mod enemies;
mod entity;
mod player;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct CorePlugins;

#[butler_plugin]
pub struct Core;

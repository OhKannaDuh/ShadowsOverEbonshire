use bevy_simple_tilemap::plugin::SimpleTileMapPlugin;

use crate::prelude::*;

mod world_generator;
pub(crate) use world_generator::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct World;

#[add_plugin(to_group = CorePlugins)]
struct WorldPlugin;

#[butler_plugin]
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleTileMapPlugin);
    }
}

mod systems;

use crate::prelude::*;

mod world_generator;
use bevy_ecs_tilemap::TilemapPlugin;
pub(crate) use world_generator::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct World;

#[add_plugin(to_group = CorePlugins)]
struct WorldPlugin;

#[butler_plugin]
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
    }
}

mod biome;
pub use biome::*;
mod systems;
mod world_gen_tester;

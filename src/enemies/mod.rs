use crate::prelude::*;

use crate::actor::Actor;

mod contact_damage;
pub(crate) use contact_damage::*;

mod enemy_ai;
pub(crate) use enemy_ai::*;

mod velocity;
pub(crate) use velocity::*;

mod collision;
pub(crate) use collision::*;

mod assets;
pub(crate) use assets::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor, ContactDamage, EnemyAi, Velocity)]
struct Enemy;

#[add_plugin(to_group = CorePlugins)]
struct EnemyPlugin;

#[butler_plugin]
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Loading).load_collection::<SlimeAssets>(),
        );
    }
}

mod systems;

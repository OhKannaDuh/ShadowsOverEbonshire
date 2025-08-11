mod prelude;
use crate::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    // MainMenu,
    InGame,
}

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

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct GameplayPlugins;

pub struct Core;

#[butler_plugin]
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            use bevy::{
                diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
                remote::{RemotePlugin, http::RemoteHttpPlugin},
            };

            app.add_plugins((
                RemotePlugin::default(),
                RemoteHttpPlugin::default(),
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}

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

pub struct Core;

#[butler_plugin]
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("VS Alpha"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

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

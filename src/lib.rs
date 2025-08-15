mod prelude;

use crate::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    // MainMenu,
    GeneratingMap,
    InGame,
}

// Components
mod animated_sprite;
mod weapon;

// Systems
mod camera;
mod input;
mod world;

// Entities
mod actor;
mod enemy;
mod entity;
mod player;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct CorePlugins;

#[butler_plugin_group]
// #[add_plugin(to_plugin = Core)]
pub(crate) struct EntityPlugins;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct RenderingPlugins;

pub struct Core;

#[butler_plugin]
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::remote::RemotePlugin::default(),
                bevy::remote::http::RemoteHttpPlugin::default(),
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                bevy::diagnostic::EntityCountDiagnosticsPlugin,
                bevy::diagnostic::SystemInformationDiagnosticsPlugin,
                // bevy::render::diagnostic::RenderDiagnosticsPlugin,
                iyes_perf_ui::PerfUiPlugin,
            ));

            fn add_performance_ui(mut commands: Commands) {
                commands.spawn(iyes_perf_ui::prelude::PerfUiAllEntries::default());
            }

            app.add_systems(Startup, add_performance_ui);
        }
    }
}

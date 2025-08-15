use bevy::{
    DefaultPlugins,
    prelude::*,
    render::texture::ImagePlugin,
    utils::default,
    window::{Window, WindowPlugin},
};

use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("VS Alpha"),
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<shadows_over_ebonshire::GameState>()
        .add_loading_state(
            LoadingState::new(shadows_over_ebonshire::GameState::Loading)
                .continue_to_state(shadows_over_ebonshire::GameState::GeneratingMap),
        )
        .add_plugins(shadows_over_ebonshire::Core)
        .run();
}

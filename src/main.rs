use bevy::{
    DefaultPlugins,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    render::texture::ImagePlugin,
    window::{Window, WindowPlugin},
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("VS Alpha"),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        RemotePlugin::default(),
        RemoteHttpPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        LogDiagnosticsPlugin::default(),
    ));

    app.add_plugins(survivor_like::Core);
    app.run();
}

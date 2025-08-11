use bevy::{
    DefaultPlugins,
    prelude::*,
    render::texture::ImagePlugin,
    window::{Window, WindowPlugin},
};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("VS Alpha"),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest()),));

    app.add_plugins(survivor_like::Core);
    app.run();
}

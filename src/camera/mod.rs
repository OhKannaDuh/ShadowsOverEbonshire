use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Camera2d, Name::new("Main Camera"))]
struct MainCamera;

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct CameraPlugin;

mod systems;

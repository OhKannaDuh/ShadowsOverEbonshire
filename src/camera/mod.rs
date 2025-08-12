use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Camera2d, Name::new("Main Camera"))]
pub struct MainCamera {
    pub base_speed: f32,
    pub max_speed: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Name::new("Camera Focus"))]
pub struct CameraFocus(bool);

impl Default for CameraFocus {
    fn default() -> Self {
        CameraFocus(true)
    }
}

#[butler_plugin]
#[add_plugin(to_group = RenderingPlugins)]
struct CameraPlugin;

mod systems;

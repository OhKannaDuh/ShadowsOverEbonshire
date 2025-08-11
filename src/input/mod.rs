use crate::prelude::*;

mod action;
pub(crate) use action::*;

#[add_plugin(to_group = CorePlugins)]
pub struct InputPlugin;

#[butler_plugin]
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
    }
}

mod systems;

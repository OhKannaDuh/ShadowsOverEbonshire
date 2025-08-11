use crate::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[insert_state(plugin = Core)]
pub enum GameState {
    #[default]
    Loading,
    // MainMenu,
    InGame,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[insert_state(plugin = Core)]
pub enum MainLoadingState {
    #[default]
    Assets,
    WorldData,
    Done,
}

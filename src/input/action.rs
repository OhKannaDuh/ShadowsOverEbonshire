use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
}

impl Action {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default().with_dual_axis(Self::Move, VirtualDPad::wasd())
    }
}

use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[actionlike(DualAxis)]
pub enum Action {
    Move,
}

impl Action {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default().with_dual_axis(Self::Move, VirtualDPad::wasd())
    }
}

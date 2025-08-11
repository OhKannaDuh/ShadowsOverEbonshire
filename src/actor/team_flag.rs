use crate::prelude::*;

#[derive(Reflect, Debug, Default)]
pub enum Team {
    #[default]
    Neutral,
    Player,
    Enemy,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct TeamFlag(pub Team);

impl Default for TeamFlag {
    fn default() -> Self {
        TeamFlag(Team::Neutral)
    }
}

use crate::prelude::*;

#[derive(Reflect, Debug, PartialEq)]
pub enum EnemyAiType {
    Basic,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct EnemyAi(pub EnemyAiType);

impl Default for EnemyAi {
    fn default() -> Self {
        EnemyAi(EnemyAiType::Basic)
    }
}

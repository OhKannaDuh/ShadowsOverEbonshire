use crate::enemies::*;
use kd_tree::{KdPoint, KdTree};

#[derive(Component)]
pub struct Collision {
    pub pos: Vec2,
    pub entity: Entity,
}

#[derive(Resource)]
#[insert_resource(plugin = EnemyPlugin)]
pub struct EnemyKdTree(pub KdTree<Collision>);

impl KdPoint for Collision {
    type Scalar = f32;
    type Dim = typenum::U2;
    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

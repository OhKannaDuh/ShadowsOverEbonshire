use kd_tree::KdTree;

use crate::actor::*;
use crate::enemies::*;
use crate::player::*;

mod ai;
mod despawner;
mod spawner;

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn move_enemies(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn apply_contact_damage(
    mut player_query: Query<(&Transform, &Aabb, &mut Health), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(&Transform, &Aabb, &ContactDamage), (With<Enemy>, Without<Player>)>,
    tree: Res<EnemyKdTree>,
    time: Res<Time>,
) {
    fn aabb_intersects(a: &Aabb, a_pos: Vec3, b: &Aabb, b_pos: Vec3) -> bool {
        let a_center = a.center + Vec3A::new(a_pos.x, a_pos.y, a_pos.z);
        let b_center = b.center + Vec3A::new(b_pos.x, b_pos.y, b_pos.z);

        let a_half = a.half_extents;
        let b_half = b.half_extents;

        let delta = (a_center - b_center).abs();

        (delta.x <= a_half.x + b_half.x)
            && (delta.y <= a_half.y + b_half.y)
            && (delta.z <= a_half.z + b_half.z)
    }

    let (player_transform, player_aabb, mut player_health) =
        player_query.single_mut().expect("No player found");

    let search_radius = (player_aabb.half_extents.x.max(player_aabb.half_extents.y)) * 2.0 + 50.0;

    let player_pos_2d = [
        player_transform.translation.x,
        player_transform.translation.y,
    ];
    let nearby_enemies = tree.0.within_radius(&player_pos_2d, search_radius);

    for enemy_collision in nearby_enemies {
        if let Ok((enemy_transform, enemy_aabb, contact_damage)) =
            enemy_query.get(enemy_collision.entity)
        {
            if aabb_intersects(
                enemy_aabb,
                enemy_transform.translation,
                player_aabb,
                player_transform.translation,
            ) {
                player_health.current -= contact_damage.0 * time.delta_secs();
            }
        }
    }
}

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&GlobalTransform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (gt, e) in enemy_query.iter() {
        items.push(Collision {
            entity: e,
            pos: gt.translation().truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

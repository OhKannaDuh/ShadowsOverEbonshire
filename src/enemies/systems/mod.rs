use std::time::Duration;

use bevy::platform::collections::HashMap;
use bevy::time::common_conditions::on_timer;
use kd_tree::KdTree;

use crate::actor::*;
use crate::enemies::*;
use crate::player::*;

mod ai;
mod spawner;

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn update_enemy_ai(
    mut params: ParamSet<(
        Query<
            (Entity, &mut Transform, &EnemyAi, &Speed, &mut Velocity),
            (With<Enemy>, Without<Player>),
        >,
        Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    )>,
    player_query: Query<&Transform, With<Player>>,
    tree: Res<EnemyKdTree>,
    time: Res<Time>,
) {
    let player_transform = player_query.single().expect("No player found");

    // Cache enemy positions for neighbor lookups
    let enemy_transforms: HashMap<Entity, Vec3> = params
        .p1()
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    let separation_distance = 30.0;
    let separation_strength = 100.0;
    let max_speed = 200.0;
    let smoothing_factor = 0.15;

    for (entity, mut transform, enemy_ai, speed, mut velocity) in params.p0().iter_mut() {
        match enemy_ai.0 {
            EnemyAiType::Basic => {
                let pos_2d = [transform.translation.x, transform.translation.y];
                let neighbors = tree.0.within_radius(&pos_2d, separation_distance);

                // Separation as Vec3, ignoring Z axis (assuming movement on XY plane)
                let mut separation = Vec3::ZERO;
                let mut neighbor_count = 0;

                for neighbor in neighbors {
                    if neighbor.entity == entity {
                        continue;
                    }
                    if let Some(neighbor_pos) = enemy_transforms.get(&neighbor.entity) {
                        let to_me = transform.translation - *neighbor_pos;
                        let dist = to_me.length();
                        if dist > 0.0 && dist < separation_distance {
                            // Weighted repulsion, ignore Z
                            let strength = (1.0 - (dist / separation_distance)).powi(2);
                            let to_me_xy = Vec3::new(to_me.x, to_me.y, 0.0).normalize() * strength;
                            separation += to_me_xy;
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count > 0 {
                    separation /= neighbor_count as f32;
                    separation = separation.normalize_or_zero() * separation_strength;
                }

                let to_player = player_transform.translation - transform.translation;
                let to_player_xy = Vec3::new(to_player.x, to_player.y, 0.0);
                let to_player_dir = if to_player_xy.length_squared() > 0.0 {
                    to_player_xy.normalize()
                } else {
                    Vec3::ZERO
                };

                let desired_velocity = (to_player_dir + separation).normalize_or_zero() * speed.0;

                // Smooth velocity interpolation
                velocity.0 = velocity.0.lerp(desired_velocity, smoothing_factor);

                // Clamp max speed in XY plane (keep Z velocity as is, or zero)
                let horizontal_speed = Vec3::new(velocity.0.x, velocity.0.y, 0.0);
                let speed_len = horizontal_speed.length();
                if speed_len > max_speed {
                    let clamped = horizontal_speed.normalize() * max_speed;
                    velocity.0.x = clamped.x;
                    velocity.0.y = clamped.y;
                }

                // Apply velocity to position (ignore Z movement)
                transform.translation.x += velocity.0.x * time.delta_secs();
                transform.translation.y += velocity.0.y * time.delta_secs();
            }
        }
    }
}

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = in_state(GameState::InGame))]
fn apply_contact_damage(
    mut player_query: Query<(&Transform, &Aabb, &mut Health), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(&Transform, &Aabb, &ContactDamage), (With<Enemy>, Without<Player>)>,
    tree: Res<EnemyKdTree>,
    time: Res<Time>,
) {
    return;
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

#[add_system(schedule = Update, plugin = EnemyPlugin, run_if = on_timer(Duration::from_secs_f32(0.2)), run_if = in_state(GameState::InGame))]
fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (t, e) in enemy_query.iter() {
        items.push(Collision {
            entity: e,
            pos: t.translation.truncate(),
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

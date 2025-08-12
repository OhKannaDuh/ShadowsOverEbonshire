use bevy::platform::collections::HashSet;

use crate::actor::Health;
use crate::enemies::Enemy;
use crate::enemies::EnemyKdTree;
use crate::prelude::*;
use crate::weapons::*;

#[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
fn spawn_orbit_and_lunge_satellites(
    mut commands: Commands,
    query: Query<(Entity, &OrbitAndLungeWeapon), Added<OrbitAndLungeWeapon>>,
) {
    for (weapon_entity, weapon_data) in query.iter() {
        info!(
            "Adding orbit and lunge satellites for weapon: {}",
            weapon_entity
        );
        for i in 0..weapon_data.satellite_count {
            let satellite = commands
                .spawn((
                    Name::new(format!("Orbit and Lunge Weapon Satellite {}", i)),
                    OrbitAndLungeSatellite {
                        index: i,
                        weapon: weapon_entity,
                        state: LungeState::Idle,
                        progress: 0.0,
                        cooldown_timer: Timer::from_seconds(
                            weapon_data.lunge_cooldown,
                            TimerMode::Once,
                        ),
                        lunge_target: None,
                        hit_entities_this_lunge: HashSet::new(),
                    },
                    Sprite {
                        image: weapon_data.satellite_image.clone(),
                        ..default()
                    },
                    // ShowAabbGizmo {
                    //     color: Some(Color::srgb(1.0, 0.0, 1.0)),
                    // },
                    Transform::default(),
                    GlobalTransform::default(),
                ))
                .id();

            commands.entity(weapon_entity).add_child(satellite);
        }
    }
}

#[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
fn update_orbit_and_lunge_satellites(
    time: Res<Time>,
    tree: Res<EnemyKdTree>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    mut query: Query<(&mut Transform, &mut OrbitAndLungeSatellite)>,
    weapon_query: Query<(&OrbitAndLungeWeapon, &GlobalTransform)>,
) {
    fn rotation_towards(from_world: Vec3, to_world: Vec3) -> Quat {
        let dir = (to_world - from_world).truncate();
        if dir.length_squared() > 0.0001 {
            Quat::from_rotation_z(Vec2::new(-dir.y, -dir.x).angle_to(Vec2::X))
        } else {
            Quat::IDENTITY
        }
    }

    for (mut transform, mut satellite) in query.iter_mut() {
        if let Ok((weapon, weapon_global)) = weapon_query.get(satellite.weapon) {
            satellite.cooldown_timer.tick(time.delta());

            let dir_factor = match weapon.orbit_direction {
                OrbitDirection::Clockwise => -1.0,
                OrbitDirection::CounterClockwise => 1.0,
            };

            let base_angle =
                (satellite.index as f32) * std::f32::consts::TAU / weapon.satellite_count as f32;

            let orbit_angle = time.elapsed_secs() * weapon.orbit_speed * dir_factor + base_angle;

            let local_orbit_offset = Vec3::new(
                weapon.orbit_radius * orbit_angle.cos(),
                weapon.orbit_radius * orbit_angle.sin(),
                0.0,
            );

            let weapon_pos_world = weapon_global.translation();

            let orbit_pos_world = weapon_pos_world + local_orbit_offset;

            match satellite.state {
                LungeState::Idle => {
                    let pos_2d = [orbit_pos_world.x, orbit_pos_world.y];
                    let nearby_enemies = tree.0.within_radius(&pos_2d, weapon.lunge_range);

                    let nearest_enemy_pos = nearby_enemies
                        .iter()
                        .filter_map(|enemy_collision| enemy_query.get(enemy_collision.entity).ok())
                        .min_by_key(|enemy_tf| {
                            let enemy_pos = enemy_tf.translation();
                            ((enemy_pos.x - orbit_pos_world.x).powi(2)
                                + (enemy_pos.y - orbit_pos_world.y).powi(2))
                            .sqrt()
                            .to_bits()
                        })
                        .map(|enemy_tf| enemy_tf.translation());

                    if let Some(target_pos_world) = nearest_enemy_pos {
                        satellite.lunge_target = Some(target_pos_world);
                        satellite.state = LungeState::LungingOut;
                        satellite.progress = 0.0;
                        satellite.cooldown_timer.reset();
                        satellite.hit_entities_this_lunge.clear();
                    } else {
                        transform.translation = local_orbit_offset;
                        transform.rotation = rotation_towards(orbit_pos_world, weapon_pos_world);
                    }
                }
                LungeState::LungingOut => {
                    if let Some(target_pos_world) = satellite.lunge_target {
                        satellite.progress += time.delta_secs() / (weapon.lunge_cooldown / 2.0);
                        if satellite.progress >= 1.0 {
                            satellite.progress = 1.0;
                            satellite.state = LungeState::Returning;
                        }

                        let lunge_pos_world =
                            orbit_pos_world.lerp(target_pos_world, satellite.progress);
                        transform.translation = lunge_pos_world - weapon_pos_world;
                        transform.rotation = rotation_towards(lunge_pos_world, target_pos_world);
                    } else {
                        satellite.state = LungeState::Idle;
                    }
                }
                LungeState::Returning => {
                    if let Some(target_pos_world) = satellite.lunge_target {
                        satellite.progress -= time.delta_secs() / (weapon.lunge_cooldown / 2.0);
                        if satellite.progress <= 0.0 {
                            satellite.progress = 0.0;
                            satellite.state = LungeState::Idle;
                            satellite.lunge_target = None;
                            satellite.cooldown_timer.reset();
                        }

                        let return_pos_world =
                            target_pos_world.lerp(orbit_pos_world, 1.0 - satellite.progress);
                        transform.translation = return_pos_world - weapon_pos_world;
                        transform.rotation = rotation_towards(return_pos_world, weapon_pos_world);
                    } else {
                        satellite.state = LungeState::Idle;
                    }
                }
            }
        }
    }
}

#[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
fn apply_orbit_and_lunge_weapon_damage(
    tree: Res<EnemyKdTree>,
    mut weapon_query: Query<&mut OrbitAndLungeWeapon>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    mut satellite_query: Query<(
        &GlobalTransform,
        &Aabb,
        &mut OrbitAndLungeSatellite,
        &GlobalTransform,
    )>,
) {
    for (global_transform, aabb, mut satellite, weapon_parent_global_transform) in
        satellite_query.iter_mut()
    {
        if satellite.state != LungeState::LungingOut {
            continue;
        }

        let Ok(weapon) = weapon_query.get_mut(satellite.weapon) else {
            continue;
        };

        let sat_pos = global_transform.translation();
        let sat_pos_2d = [sat_pos.x, sat_pos.y];
        let radius = aabb.half_extents.max_element();

        let nearby_enemies = tree.0.within_radius(&sat_pos_2d, radius);

        for enemy_collision in nearby_enemies {
            let enemy_entity = enemy_collision.entity;

            if satellite.hit_entities_this_lunge.contains(&enemy_entity) {
                continue;
            }

            if let Ok(mut health) = enemy_query.get_mut(enemy_entity) {
                health.current -= weapon.lunge_damage;

                satellite.hit_entities_this_lunge.insert(enemy_entity);
            }
        }
    }
}

// #[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
// fn draw_lunge_ranges(
//     mut gizmos: Gizmos,
//     satellite_query: Query<(&GlobalTransform, &OrbitAndLungeSatellite)>,
//     weapon_query: Query<&OrbitAndLungeWeapon>,
// ) {
//     for (global_transform, satellite) in &satellite_query {
//         if let Ok(weapon) = weapon_query.get(satellite.weapon) {
//             gizmos.circle_2d(
//                 global_transform.translation().truncate(),
//                 weapon.lunge_range,
//                 Color::srgb(0.0, 1.0, 0.0),
//             );
//         }
//     }
// }

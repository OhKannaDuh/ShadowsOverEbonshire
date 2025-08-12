use crate::actor::*;
use crate::enemies::*;
use crate::prelude::*;
use crate::weapons::*;

#[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
fn spawn_orbit_satellites(
    mut commands: Commands,
    query: Query<(Entity, &OrbitWeapon), Added<OrbitWeapon>>,
) {
    for (weapon_entity, weapon_data) in query.iter() {
        info!("Adding orbit satellites for weapon: {}", weapon_entity);
        for i in 0..weapon_data.satellite_count {
            let satellite = commands
                .spawn((
                    Name::new(format!("Orbit Weapon Satellite {}", i)),
                    OrbitWeaponSatellite {
                        index: i,
                        weapon: weapon_entity,
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
fn update_orbit_satellites(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &OrbitWeaponSatellite)>,
    weapon_query: Query<&OrbitWeapon>,
) {
    for (mut transform, satellite) in query.iter_mut() {
        if let Ok(weapon_data) = weapon_query.get(satellite.weapon) {
            let dir_factor = match weapon_data.orbit_direction {
                OrbitDirection::Clockwise => -1.0,
                OrbitDirection::CounterClockwise => 1.0,
            };

            let base_angle = (satellite.index as f32) * std::f32::consts::TAU
                / weapon_data.satellite_count as f32;

            let orbit_angle =
                time.elapsed_secs() * weapon_data.orbit_speed * dir_factor + base_angle;

            let pos = Vec3::new(
                weapon_data.orbit_radius * orbit_angle.cos(),
                weapon_data.orbit_radius * orbit_angle.sin(),
                0.0,
            );
            transform.translation = pos;

            transform.rotation = Quat::from_rotation_z(pos.yx().angle_to(Vec2::X));
        }
    }
}

#[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
fn apply_orbit_weapon_damage(
    tree: Res<EnemyKdTree>,
    satellite_query: Query<(&GlobalTransform, &Aabb, &OrbitWeaponSatellite)>,
    mut weapon_query: Query<&mut OrbitWeapon>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
) {
    for (global_transform, aabb, satellite) in satellite_query.iter() {
        if let Ok(mut weapon) = weapon_query.get_mut(satellite.weapon) {
            let cooldown = weapon.contact_cooldown_per_entity;

            let pos = global_transform.translation();
            let player_pos_2d = [pos.x, pos.y];

            let radius = aabb.half_extents.max_element();

            let nearby_enemies = tree.0.within_radius(&player_pos_2d, radius);

            for enemy_collision in nearby_enemies {
                let enemy_entity = enemy_collision.entity;

                // Check cooldown per enemy
                let on_cooldown = weapon
                    .contact_cooldown_map
                    .get(&enemy_entity)
                    .map(|timer| !timer.finished())
                    .unwrap_or(false);

                if on_cooldown {
                    continue;
                }

                if let Ok(mut health) = enemy_query.get_mut(enemy_entity) {
                    health.current -= weapon.contact_damage;

                    weapon
                        .contact_cooldown_map
                        .insert(enemy_entity, Timer::from_seconds(cooldown, TimerMode::Once));
                }
            }
        }
    }
}

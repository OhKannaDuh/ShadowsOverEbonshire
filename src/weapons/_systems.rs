// use crate::actor::*;
// use crate::enemies::*;
// use crate::player::*;
// use crate::prelude::*;
// use crate::weapons::*;

// #[add_system(schedule = Update, plugin = WeaponPlugin, run_if = in_state(GameState::InGame))]
// fn weapon_attack_system(
//     time: Res<Time>,
//     player_query: Query<(&Transform, &EquippedWeapons), With<Player>>,
//     mut weapons: Query<(&mut Weapon, &Transform)>,
//     enemy_kd_tree: Res<EnemyKdTree>,
//     mut commands: Commands,
// ) {
//     let now = time.elapsed_secs();

//     if let Ok((player_transform, equipped)) = player_query.single() {
//         for &weapon_entity in &equipped.0 {
//             if let Ok((mut weapon, weapon_transform)) = weapons.get_mut(weapon_entity) {
//                 if now - weapon.last_attack_time < weapon.cooldown {
//                     continue;
//                 }

//                 match &weapon.weapon_type {
//                     WeaponType::Orbit(data) => {
//                         // let satellite_index = (weapon_entity.id() % data.satellite_count) as f32;
//                         // let angle = (now * data.orbit_speed
//                         //     + satellite_index
//                         //         * (2.0 * std::f32::consts::PI / data.satellite_count as f32))
//                         //     % (2.0 * std::f32::consts::PI);
//                     }
//                     WeaponType::OrbitAndLunge(data) => {}
//                     WeaponType::DirectionalMelee(data) => {}
//                     WeaponType::TargettedProjectile(data) => {}
//                     WeaponType::RandomProjectile(data) => {}
//                     WeaponType::AoeAroundPlayer(data) => {}
//                 }

//                 weapon.last_attack_time = now;
//             }
//         }
//     }
// }

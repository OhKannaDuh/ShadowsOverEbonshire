mod weapon;
pub(crate) use weapon::*;

mod orbit_common;
pub(crate) use orbit_common::*;

mod orbit_weapon;
pub(crate) use orbit_weapon::*;

mod orbit_and_lunge_weapon;
pub(crate) use orbit_and_lunge_weapon::*;

// use bevy::platform::collections::HashMap;

// #[derive(Debug, Reflect)]
// pub enum WeaponType {
//     Orbit(OrbitWeaponData),
//     OrbitAndLunge(OrbitAndLungeWeaponData),
//     DirectionalMelee(DirectionalMeleeWeaponData),
//     TargettedProjectile(TargettedProjectileWeaponData),
//     RandomProjectile(RandomProjectileWeaponData),
//     AoeAroundPlayer(AoeAroundPlayerWeaponData),
// }

// #[derive(Debug, Reflect)]
// pub struct OrbitWeaponData {
//     pub orbit_speed: f32,
//     pub orbit_radius: f32,
//     pub satellite_count: usize,
//     pub contact_damage: f32,
//     pub contact_cooldown_per_entity: f32,
//     pub contact_cooldown_map: HashMap<Entity, Timer>,
// }

// #[derive(Debug, Reflect)]
// pub struct OrbitAndLungeWeaponData {
//     pub orbit_speed: f32,
//     pub orbit_radius: f32,
//     pub satellite_count: usize,
//     pub lunge_damage: f32,
//     pub lunge_range: f32,
//     pub lunge_cooldown: f32,
// }

// #[derive(Debug, Reflect)]
// pub struct DirectionalMeleeWeaponData {
//     pub damage: f32,
//     pub range: f32,            // How far the attack reaches in the given direction
//     pub attack_angle_deg: f32, // Cone angle or arc of attack (e.g. 60° cone)
//     pub attack_rate: f32,      // Attacks per second
//     pub direction_mode: DirectionMode,
// }

// #[derive(Debug, Reflect)]
// pub enum DirectionMode {
//     Moving,
//     Left,
//     Right,
//     Up,
//     Down,
//     Custom(Vec3),
// }

// #[derive(Debug, Reflect)]
// pub struct TargettedProjectileWeaponData {
//     pub damage: f32,
//     pub projectile_speed: f32,
//     pub projectile_lifetime: f32,
//     pub attack_rate: f32,
//     pub max_targets: usize,
//     pub homing: bool,
//     pub spread_angle_deg: f32,
// }

// #[derive(Debug, Reflect)]
// pub struct RandomProjectileWeaponData {
//     pub damage: f32,
//     pub projectile_speed: f32,
//     pub projectile_lifetime: f32,
//     pub attack_rate: f32,
//     pub max_projectiles_per_shot: usize,
//     pub spread_angle_deg: f32,
//     pub firing_arc_deg: f32,
// }

// #[derive(Debug, Reflect)]
// pub struct AoeAroundPlayerWeaponData {
//     pub damage: f32,
//     pub radius: f32,
//     pub attack_rate: f32,
//     pub duration: Option<f32>,
//     pub damage_over_time: bool,
//     pub tick_interval: Option<f32>,
// }

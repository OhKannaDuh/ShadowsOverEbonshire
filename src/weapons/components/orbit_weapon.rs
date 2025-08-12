use bevy::platform::collections::HashMap;

use crate::prelude::*;
use crate::weapons::components::*;

#[derive(Component)]
#[require(Sprite)]
pub struct OrbitWeaponSatellite {
    pub index: usize,
    pub weapon: Entity,
}

#[derive(Component, Debug, Reflect)]
#[require(Weapon)]
pub struct OrbitWeapon {
    pub orbit_speed: f32,
    pub orbit_radius: f32,
    pub orbit_direction: OrbitDirection,
    pub satellite_count: usize,
    pub satellite_image: Handle<Image>,
    pub contact_damage: f32,
    pub contact_cooldown_per_entity: f32,
    pub contact_cooldown_map: HashMap<Entity, Timer>,
}

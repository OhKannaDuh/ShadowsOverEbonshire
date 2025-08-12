use bevy::platform::collections::HashSet;

use crate::prelude::*;
use crate::weapons::components::*;

#[derive(Component)]
#[require(Sprite)]
pub struct OrbitAndLungeSatellite {
    pub index: usize,
    pub weapon: Entity,
    pub state: LungeState,
    pub progress: f32,
    pub cooldown_timer: Timer,
    pub lunge_target: Option<Vec3>,
    pub hit_entities_this_lunge: HashSet<Entity>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LungeState {
    Idle,
    LungingOut,
    Returning,
}

#[derive(Component, Debug, Reflect)]
#[require(Weapon)]
pub struct OrbitAndLungeWeapon {
    pub orbit_speed: f32,
    pub orbit_radius: f32,
    pub orbit_direction: OrbitDirection,
    pub satellite_count: usize,
    pub satellite_image: Handle<Image>,
    pub lunge_damage: f32,
    pub lunge_range: f32,
    pub lunge_cooldown: f32,
}

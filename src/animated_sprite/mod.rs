use bevy::platform::collections::HashMap;

use crate::prelude::*;

#[derive(Reflect, Debug, Clone)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub durations: Vec<f32>,
}

impl Animation {
    pub fn new(frames: Vec<usize>, durations: Vec<f32>) -> Self {
        assert_eq!(
            frames.len(),
            durations.len(),
            "Frames and durations must match in length"
        );

        Self { frames, durations }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Sprite)]
pub struct AnimatedSprite {
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
    pub current_frame: usize,
    pub timer: Timer,
}

#[butler_plugin]
#[add_plugin(to_group = CorePlugins)]
struct AnimatedSpritePlugin;

mod systems;

use crate::animated_sprite::*;

#[add_system(schedule = Update, plugin = AnimatedSpritePlugin, run_if = in_state(GameState::InGame))]
fn animate_sprite(mut query: Query<(&mut AnimatedSprite, &mut Sprite)>, time: Res<Time>) {
    for (mut animated, mut sprite) in query.iter_mut() {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        let (frame_count, frame_index, frame_duration) = {
            let animation = match animated.animations.get(&animated.current_animation) {
                Some(a) => a,
                None => continue,
            };

            (
                animation.frames.len(),
                animation.frames[animated.current_frame],
                animation.durations[animated.current_frame],
            )
        };

        animated.timer.tick(time.delta());

        if animated.timer.finished() {
            animated.current_frame = (animated.current_frame + 1) % frame_count;
            atlas.index = frame_index;
            animated
                .timer
                .set_duration(std::time::Duration::from_secs_f32(frame_duration));
        }
    }
}

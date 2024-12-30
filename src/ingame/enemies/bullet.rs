use bevy::prelude::*;

use crate::AppState;
use crate::ingame::CAMERA_SPEED;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: f32,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: f32) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: f32) -> Timer {
        Timer::from_seconds(fps, TimerMode::Repeating)
    }
}

fn animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    time: Res<Time>,
) {
    // println!("enemies.bullet: animation");
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(speed: Vec2) -> Self {
        Self(speed)
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("enemies.bullet: apply_velocity");
    for (mut transform, velocity) in &mut query {
        // movement
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
        transform.translation.y += CAMERA_SPEED;
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                animation,
                apply_velocity,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                crate::ingame::enemies::fighter::bullet::check_for_hit,
                crate::ingame::enemies::torpedo::bullet::check_for_hit,
                crate::ingame::player::ship::damage_life,
                crate::ingame::player::ship::damage_animation,
                crate::ingame::player::ship::damage_despawn,
            ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}

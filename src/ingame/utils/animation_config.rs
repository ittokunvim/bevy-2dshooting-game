use bevy::prelude::*;

use crate::AppState;
use crate::ingame::player::Player;

#[derive(Component)]
pub struct AnimationConfig {
    name: AnimationName,
    pub first_sprite_index: usize,
    last_sprite_index: usize,
    fps: f32,
    frame_timer: Timer,
}

#[derive(PartialEq)]
pub enum AnimationName {
    Bullet,
    Despawn,
    PlayerDamage,
}

impl AnimationConfig {
    pub fn new(name: AnimationName, first: usize, last: usize, fps: f32) -> Self {
        Self {
            name,
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

fn bullet_animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    time: Res<Time>,
) {
    for (mut config, mut sprite) in &mut query {
        if config.name != AnimationName::Bullet { continue }

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

fn despawn_animation(
    mut commands: Commands,
    mut query: Query<(&mut AnimationConfig, Entity, &mut Sprite), With<AnimationConfig>>,
    time: Res<Time>,
) {
    for (mut config, entity, mut sprite) in &mut query {
        if config.name != AnimationName::Despawn { continue }

        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    commands.entity(entity).despawn();
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

fn player_damage_animation(
    mut query: Query<(&Player, &AnimationConfig, &mut Sprite)>,
) {
    let Ok((player, config, mut sprite)) = query.get_single_mut() else { return };

    if config.name != AnimationName::PlayerDamage { return }

    if let Some(atlas) = &mut sprite.texture_atlas {
        match player.hp {
            6 => atlas.index = 1,
            4 => atlas.index = 2,
            2 => atlas.index = 3,
            _ => {},
        }
    }
}

pub struct AnimationConfigPlugin;

impl Plugin for AnimationConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                bullet_animation,
                despawn_animation,
                player_damage_animation,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

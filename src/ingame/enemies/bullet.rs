use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
};

use crate::{
    WINDOW_SIZE,
    AppState,
    MyCamera,
};
use crate::ingame::{
    CAMERA_SPEED,
    PLAYER_SIZE,
    PlayerDamageEvent,
    PlayerShip,
};

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    last_sprite_index: usize,
    fps: f32,
    frame_timer: Timer,
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

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Bullet {
    pub size: Vec2,
}

impl Bullet {
    pub fn new(
        size: Vec2,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        first_index: usize,
        translation: Vec3,
        degrees: f32,
        scale: Vec3,
    ) -> (Self, Sprite, Transform) {
        (
            Self { size, },
            Self::sprite(image, layout, first_index),
            Self::transform(translation, degrees, scale)
        )
    }

    fn sprite(
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
    ) -> Sprite {
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout,
                index,
            }
        )
    }

    fn transform(
        translation: Vec3,
        degrees: f32,
        scale: Vec3,
    ) -> Transform {
        Transform {
            translation,
            rotation: Quat::from_rotation_z(degrees.to_radians()),
            scale,
        }
    }
}

fn check_for_hit(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(Entity, &Bullet, &Transform), (With<Bullet>, Without<PlayerShip>)>,
    player_query: Query<&Transform, (With<PlayerShip>, Without<Bullet>)>,
) {
    // println!("enemies.bullet: check_for_hit");
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.xy();

    for (bullet_entity, bullet, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
            .intersects(&Aabb2d::new(player_pos, PLAYER_SIZE / 2.0));

        if collision {
            // damage player
            events.send_default();
            // despawn bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_offscreen(
    mut commands: Commands,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<MyCamera>)>,
) {
    // println!("enemies.bullet: check_for_offscreen");
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = camera_transform.translation.y;

    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_y = bullet_transform.translation.y;
        // check off screen
        if bullet_y <= camera_y - WINDOW_SIZE.y / 2.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bullet>>,
) {
    // println!("enemies.bullet: despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                animation,
                apply_velocity,
                // check_for_hit,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                check_for_hit,
                crate::ingame::player::ship::damage_life,
                crate::ingame::player::ship::damage_animation,
                crate::ingame::player::ship::damage_despawn,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
        ;
    }
}

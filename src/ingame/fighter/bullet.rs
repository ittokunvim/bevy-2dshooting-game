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
    GRID_SIZE,
    CAMERA_SPEED,
};
use crate::ingame::player::{
    PlayerDamageEvent,
    Player,
};
use crate::ingame::fighter::Fighter;

const PATH_IMAGE: &str = "bevy-2dshooting-game/fighter-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(2.0);
const DIRECTION: Vec2 = Vec2::new(0.0, -1.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(8.0, 32.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Component)]
struct AnimationConfig {
    pub first_sprite_index: usize,
    last_sprite_index: usize,
    fps: f32,
    frame_timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
#[require(Sprite, Transform)]
struct Bullet {
    size: Vec2,
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut fighter_query: Query<(&mut Fighter, &Transform), With<Fighter>>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    for (mut fighter, fighter_transform) in &mut fighter_query {
        if !fighter.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let translation = Vec3::new(
            fighter_transform.translation.x, 
            fighter_transform.translation.y - GRID_SIZE * 2.0, 
            99.0,
        );

        let animation_config = AnimationConfig::new(0, 3, FPS);
        let velocity = Velocity(DIRECTION * SPEED);
        let bullet = Bullet::new(
            SIZE, 
            bullet_image.clone(), 
            texture_atlas_layout.clone(), 
            animation_config.first_sprite_index, 
            translation, 
            DEGREES, 
            SCALE,
        );
        // bullet
        commands.spawn((bullet, animation_config, velocity));
    }
}

fn animation(
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<AnimationConfig>>,
    time: Res<Time>,
) {
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

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time_step: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        // movement
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
        transform.translation.y += CAMERA_SPEED;
    }
}

fn check_for_hit(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(Entity, &Bullet, &Transform), (With<Bullet>, Without<Player>)>,
    player_query: Query<(&Transform, &Player), (With<Player>, Without<Bullet>)>,
) {
    let Ok((player_transform, player)) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.xy();

    for (bullet_entity, bullet, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
            .intersects(&Aabb2d::new(player_pos, player.size / 2.0));

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
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                shoot,
                animation,
                apply_velocity,
                check_for_hit,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
        ;
    }
}

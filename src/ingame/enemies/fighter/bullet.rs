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
    PLAYER_SIZE,
    PlayerDamageEvent,
    PlayerShip,
    FighterShip,
};
use crate::ingame::enemies::bullet;

const PATH_IMAGE: &str = "bevy-2dshooting-game/fighter-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(2.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(8.0, 32.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Component)]
pub struct Bullet;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("fighter.bullet: setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut fighter_query: Query<(&mut FighterShip, &Transform), With<FighterShip>>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    // println!("fighter.bullet: shoot");
    for (mut fighter, fighter_transform) in &mut fighter_query {
        if !fighter.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_config = bullet::AnimationConfig::new(0, 3, FPS);
        let translation = Vec3::new(
            fighter_transform.translation.x, 
            fighter_transform.translation.y - GRID_SIZE * 2.0, 
            99.0,
        );
        // bullet
        commands.spawn((
            Sprite::from_atlas_image(
                bullet_image.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_config.first_sprite_index,
                },
            ),
            Transform {
                translation,
                rotation: Quat::from_rotation_z(DEGREES.to_radians()),
                scale: SCALE,
            },
            animation_config,
            Bullet,
        ));
    }
}

fn movement(
    mut query: Query<&mut Transform, With<Bullet>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("enemy.bullet: movement");
    for mut transform in &mut query {
        transform.translation.y -= SPEED * time_step.delta().as_secs_f32();
        transform.translation.y += CAMERA_SPEED;
    }
}

pub fn check_for_hit(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<PlayerShip>)>,
    player_query: Query<&Transform, (With<PlayerShip>, Without<Bullet>)>,
) {
    // println!("torpedo.bullet: check_for_hit");
    let Ok(player_transform) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.xy();

    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, SIZE / 2.0)
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
    // println!("fighter.bullet: check_for_offscreen");
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

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bullet>>,
) {
    // println!("fighter.bullet: despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                shoot,
                bullet::animation,
                movement,
                // check_for_hit, // moved ingame/enemies/mod.rs
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), despawn)
        ;
    }
}

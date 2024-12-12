use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
};

use crate::{
    WINDOW_SIZE,
    AppState,
};
use crate::ingame::{
    GRID_SIZE,
    PATH_IMAGE_ENEMY_BULLET,
    PLAYER_SIZE,
    PlayerShip,
    EnemyShip,
    EnemyBullet,
    EnemyBulletHitEvent,
};

const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SHOOT_INTERVAL: f32 = 0.5;
const SIZE: Vec2 = Vec2::new(8.0, 32.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Resource)]
struct ShootTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("enemy.bullet: setup");
    // bullet image
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_ENEMY_BULLET);
    commands.insert_resource(BulletImage(handle));
}

fn animation(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<EnemyBullet>>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last 
                    { indices.first } else { atlas.index + 1 }
            }
        }
    }
}

fn movement(
    mut query: Query<&mut Transform, With<EnemyBullet>>,
    time_step: Res<Time<Fixed>>,
) {
    for mut transform in &mut query {
        // move bullet
        transform.translation.y -= SPEED * time_step.delta().as_secs_f32();
    }
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut timer: ResMut<ShootTimer>,
    bullet_image: Res<BulletImage>,
    enemy_query: Query<&Transform, With<EnemyShip>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() { return }

    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3, };
    let enemy_transform = enemy_query.single();
    let translation = Vec3::new(
        enemy_transform.translation.x, 
        enemy_transform.translation.y - GRID_SIZE * 2.0, 
        99.0,
    );
    // println!("enemy.bullet: shoot");
    commands.spawn((
        Sprite::from_atlas_image(
            bullet_image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation,
            scale: SCALE,
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(FPS, TimerMode::Repeating)),
        EnemyBullet,
    ));
}

fn check_bullet_hit(
    mut commands: Commands,
    mut events: EventWriter<EnemyBulletHitEvent>,
    bullet_query: Query<(Entity, &Transform), (With<EnemyBullet>, Without<PlayerShip>)>,
    player_query: Query<&Transform, (With<PlayerShip>, Without<EnemyBullet>)>,
) {
    let player_transform = player_query.single();
    let player_pos = player_transform.translation.xy();

    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, SIZE / 2.0)
            .intersects(&Aabb2d::new(player_pos, PLAYER_SIZE / 2.0));

        if collision {
            // println!("enemy.bullet: enemy bullet hit player");
            events.send_default();
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y <= -WINDOW_SIZE.y / 2.0 {
            // println!("enemy.bullet: despawn");
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShootTimer(Timer::from_seconds(
                SHOOT_INTERVAL, 
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                animation,
                movement,
                shoot,
                check_bullet_hit,
                despawn,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

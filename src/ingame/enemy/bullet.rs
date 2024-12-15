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
    PLAYER_SIZE,
    PlayerShip,
    EnemyShip,
    PlayerDamageEvent,
};

const PATH_IMAGE_ENEMY_BULLET: &str = "bevy-2dshooting-game/enemy-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(8.0, 32.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Component)]
struct Bullet;

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
    // println!("enemy.bullet: setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_ENEMY_BULLET);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut enemy_query: Query<(&mut EnemyShip, &Transform), With<EnemyShip>>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    // println!("enemy.bullet: shoot");
    for (mut enemy, enemy_transform) in &mut enemy_query {
        if !enemy.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3, };
        let translation = Vec3::new(
            enemy_transform.translation.x, 
            enemy_transform.translation.y - GRID_SIZE * 2.0, 
            99.0,
        );
        // bullet
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
            Bullet,
        ));
    }
}

fn animation(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<Bullet>>,
    time: Res<Time>,
) {
    // println!("enemy.bullet: animation");
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // animation
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last 
                    { indices.first } else { atlas.index + 1 }
            }
        }
    }
}

fn movement(
    mut query: Query<&mut Transform, With<Bullet>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("enemy.bullet: movement");
    for mut transform in &mut query {
        transform.translation.y -= SPEED * time_step.delta().as_secs_f32();
    }
}

fn check_for_hit(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<PlayerShip>)>,
    player_query: Query<&Transform, (With<PlayerShip>, Without<Bullet>)>,
) {
    // println!("enemy.bullet: check_for_hit");
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
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    // println!("enemy.bullet: despawn");
    for (entity, transform) in &query {
        if transform.translation.y <= -WINDOW_SIZE.y / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                shoot,
                animation,
                movement,
                check_for_hit,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

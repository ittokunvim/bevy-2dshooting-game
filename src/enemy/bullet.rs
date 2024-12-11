use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    GRID_SIZE,
    PATH_IMAGE_ENEMY_BULLET,
};
use crate::enemy::{
    Enemy,
    Bullet,
};

const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SHOOT_INTERVAL: f32 = 0.5;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Resource)]
struct ShootTimer(Timer);

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
    mut query: Query<(&Bullet, &mut AnimationTimer, &mut Sprite), With<Bullet>>,
    time: Res<Time>,
) {
    for (prop, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == prop.last 
                    { prop.first } else { atlas.index + 1 }
            }
        }
    }
}

fn movement(
    mut query: Query<&mut Transform, With<Bullet>>,
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
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() { return }

    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Bullet { first: 0, last: 3, };
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
    ));
}

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
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
            .add_systems(Startup, setup)
            .add_systems(Update, (
                animation,
                movement,
                shoot,
                despawn,
            ))
        ;
    }
}

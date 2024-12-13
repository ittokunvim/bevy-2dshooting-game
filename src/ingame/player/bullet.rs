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
    ENEMY_SIZE,
    PlayerShip,
    EnemyShip,
    PlayerBulletHitEvent,
};
use crate::ingame::player::ShootEvent;

const PATH_IMAGE_PLAYER_BULLET: &str = "bevy-2dshooting-game/player-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const SIZE: Vec2 = Vec2::splat(32.0);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SPEED: f32 = 512.0;
const FPS: f32 = 0.1;
const KEYCODE: KeyCode = KeyCode::Space;
const MAX_COUNT: usize = 2;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Resource, Deref, DerefMut, Debug)]
struct Remaining(usize);

#[derive(Component)]
struct PlayerBullet;

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
    println!("player.bullet: setup");
    // bullet image
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_PLAYER_BULLET);
    commands.insert_resource(BulletImage(handle));
}

fn event(
    mut events: EventWriter<ShootEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KEYCODE) { return }
    // println!("player.bullet: {:?} pressed", KEYCODE);
    events.send_default();
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut events: EventReader<ShootEvent>,
    mut remaining: ResMut<Remaining>,
    bullet_image: Res<BulletImage>,
    player_query: Query<&Transform, With<PlayerShip>>,
) {
    if events.is_empty() || **remaining <= 0 { return }
    events.clear();

    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3, };
    let Ok(player_transform) = player_query.get_single() else { return };
    let translation = Vec3::new(
        player_transform.translation.x, 
        player_transform.translation.y + GRID_SIZE * 2.0, 
        99.0,
    );
    // println!("player.bullet: shoot");
    commands.spawn((
        Sprite::from_atlas_image(
            bullet_image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_translation(translation),
        animation_indices,
        AnimationTimer(Timer::from_seconds(FPS, TimerMode::Repeating)),
        PlayerBullet,
    ));
    **remaining -= 1;
}

fn animation(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<PlayerBullet>>,
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
    mut query: Query<&mut Transform, With<PlayerBullet>>,
    time_step: Res<Time<Fixed>>,
) {
    for mut transform in &mut query {
        // move bullet
        transform.translation.y += SPEED * time_step.delta().as_secs_f32();
    }
}

fn check_bullet_hit(
    mut commands: Commands,
    mut hit_events: EventWriter<PlayerBulletHitEvent>,
    mut remaining: ResMut<Remaining>,
    bullet_query: Query<(Entity, &Transform), (With<PlayerBullet>, Without<EnemyShip>)>,
    enemy_query: Query<(Entity, &Transform), (With<EnemyShip>, Without<PlayerBullet>)>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();
        let mut is_hit_bullet = false;

        for (enemy_entity, enemy_transform) in &enemy_query {
            let enemy_pos = enemy_transform.translation.xy();
            let collision = Aabb2d::new(bullet_pos, SIZE / 2.0)
                .intersects(&Aabb2d::new(enemy_pos, ENEMY_SIZE / 2.0));

            if collision {
                // println!("player.bullet: player bullet hit enemy");
                is_hit_bullet = true;
                hit_events.send(PlayerBulletHitEvent(enemy_entity, enemy_pos));
            }
        }
        // println!("player.bullet: despawn");
        if is_hit_bullet {
            **remaining += 1;
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_bullet_offscreen(
    mut commands: Commands,
    mut remaining: ResMut<Remaining>,
    query: Query<(Entity, &Transform), With<PlayerBullet>>,
) {
    for (entity, transform) in  &query {
        if transform.translation.y >= WINDOW_SIZE.y / 2.0 {
            // println!("player.bullet: check bullet offscreen");
            **remaining += 1;
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Remaining(MAX_COUNT))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                event,
                shoot,
                animation,
                movement,
                check_bullet_hit,
                check_bullet_offscreen,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

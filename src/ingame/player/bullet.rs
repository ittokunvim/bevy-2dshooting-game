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
    FIGHTER_SIZE,
    TORPEDO_SIZE,
    FighterDamageEvent,
    TorpedoDamageEvent,
    PlayerShip,
    FighterShip,
    TorpedoShip,
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
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Bullet;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("player.bullet: setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_PLAYER_BULLET);
    commands.insert_resource(BulletImage(handle));
}

fn event(
    mut events: EventWriter<ShootEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // println!("player.bullet: event");
    if !keyboard_input.just_pressed(KEYCODE) { return }
    // send shoot event
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
    // println!("player.bullet: shoot");
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
    // bullet
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
        Bullet,
    ));
    // increase remaining bullet
    **remaining -= 1;
}

fn animation(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<Bullet>>,
    time: Res<Time>,
) {
    // println!("player.bullet: animation");
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
    mut query: Query<&mut Transform, With<Bullet>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("player.bullet: movement");
    for mut transform in &mut query {
        transform.translation.y += SPEED * time_step.delta().as_secs_f32();
        transform.translation.y += CAMERA_SPEED;
    }
}

fn check_for_hit_fighter(
    mut commands: Commands,
    mut events: EventWriter<FighterDamageEvent>,
    mut remaining: ResMut<Remaining>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<FighterShip>)>,
    fighter_query: Query<(Entity, &Transform), (With<FighterShip>, Without<Bullet>)>,
) {
    // println!("player.bullet: check_for_hit_fighter");
    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();
        let mut is_hit_bullet = false;

        for (fighter_entity, fighter_transform) in &fighter_query {
            let fighter_pos = fighter_transform.translation.xy();
            let collision = Aabb2d::new(bullet_pos, SIZE / 2.0)
                .intersects(&Aabb2d::new(fighter_pos, FIGHTER_SIZE / 2.0));

            if collision {
                // flag a player bullet hit
                is_hit_bullet = true;
                // damage fighter
                events.send(FighterDamageEvent(fighter_entity));
            }
        }
        if is_hit_bullet {
            // reduce remaining bullet
            **remaining += 1;
            // despawn player bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_hit_torpedo(
    mut commands: Commands,
    mut events: EventWriter<TorpedoDamageEvent>,
    mut remaining: ResMut<Remaining>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<FighterShip>)>,
    torpedo_query: Query<(Entity, &Transform), (With<TorpedoShip>, Without<Bullet>)>,
) {
    // println!("player.bullet: check_for_hit_torpedo");
    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();
        let mut is_hit_bullet = false;

        for (torpedo_entity, torpedo_transform) in &torpedo_query {
            let torpedo_pos = torpedo_transform.translation.xy();
            let collision = Aabb2d::new(bullet_pos, SIZE / 2.0)
                .intersects(&Aabb2d::new(torpedo_pos, TORPEDO_SIZE / 2.0));

            if collision {
                // flag a player bullet hit
                is_hit_bullet = true;
                // damage torpedo
                events.send(TorpedoDamageEvent(torpedo_entity));
            }
        }
        if is_hit_bullet {
            // reduce remaining bullet
            **remaining += 1;
            // despawn player bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_offscreen(
    mut commands: Commands,
    mut remaining: ResMut<Remaining>,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<MyCamera>)>,
) {
    // println!("player.bullet: check_for_offscreen");
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = camera_transform.translation.y;

    for (bullet_entity, bullet_transform) in  &bullet_query {
        let bullet_y = bullet_transform.translation.y;
        // check off screen
        if bullet_y >= camera_y + WINDOW_SIZE.y / 2.0 {
            // reduce remaining bullet
            **remaining += 1;
            // despawn player bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn reset_remaining(mut remaining: ResMut<Remaining>) {
    **remaining = MAX_COUNT;
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bullet>>,
) {
    // println!("player.bullet: despawn");
    for entity in &query { commands.entity(entity).despawn() }
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
                // check_for_hit,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                check_for_hit_fighter,
                crate::ingame::enemies::fighter::ship::damage,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                check_for_hit_torpedo,
                crate::ingame::enemies::torpedo::ship::damage,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), (
                reset_remaining,
                despawn,
            ))
        ;
    }
}

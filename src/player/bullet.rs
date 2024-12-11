use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    GRID_SIZE,
    PATH_IMAGE_PLAYER_BULLET,
    PATH_SOUND_SHOOT,
    PlayerShip,
    PlayerBullet,
};

const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SPEED: f32 = 512.0;
const FPS: f32 = 0.1;
const KEYCODE: KeyCode = KeyCode::Space;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Resource, Deref)]
struct ShootSound(Handle<AudioSource>);

#[derive(Event, Default)]
struct ShootEvent;

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
    // shoot sound
    let handle = asset_server.load(PATH_SOUND_SHOOT);
    commands.insert_resource(ShootSound(handle));
}

fn animation(
    mut query: Query<(&PlayerBullet, &mut AnimationTimer, &mut Sprite), With<PlayerBullet>>,
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
    mut query: Query<&mut Transform, With<PlayerBullet>>,
    time_step: Res<Time<Fixed>>,
) {
    for mut transform in &mut query {
        // move bullet
        transform.translation.y += SPEED * time_step.delta().as_secs_f32();
    }
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
    bullet_image: Res<BulletImage>,
    player_query: Query<&Transform, With<PlayerShip>>,
) {
    if events.is_empty() { return }
    events.clear();

    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = PlayerBullet { first: 0, last: 3, };
    let player_transform = player_query.single();
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
    ));
}

fn play(
    mut events: EventReader<ShootEvent>,
    mut commands: Commands,
    sound: Res<ShootSound>,
) {
    if events.is_empty() { return }
    events.clear();
    // println!("player.bullet: play");
    commands.spawn((
        AudioPlayer(sound.clone()),
        PlaybackSettings::DESPAWN,
    ));
}

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<PlayerBullet>>,
) {
    for (entity, transform) in  &query {
        if transform.translation.y >= WINDOW_SIZE.y / 2.0 {
            // println!("player.bullet: despawn");
            commands.entity(entity).despawn();
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                animation,
                movement,
                event,
                shoot,
                play,
                despawn,
            ))
        ;
    }
}

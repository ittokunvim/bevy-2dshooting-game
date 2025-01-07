use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::ops::Range;

use crate::{
    WINDOW_SIZE,
    AppState,
    Score,
    MyCamera,
};
use crate::ingame::GRID_SIZE;
use crate::ingame::fighter::FighterDespawnEvent;
use crate::ingame::torpedo::{
    PATH_IMAGE_TORPEDO as PATH_IMAGE,
    TorpedoDamageEvent,
    TorpedoDespawnEvent,
    Torpedo,
};
use crate::ingame::utils::prelude::*;

const SIZE: Vec2 = Vec2::new(128.0, 32.0);
const HP: usize = 3;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
const SPEED: f32 = 128.0;
const TIMER_RANGE: Range<f32> = 1.5..2.0;

#[derive(Resource, Deref)]
struct ShipImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(ShipImage(handle));
}

fn spawn(
    mut commands: Commands,
    mut events: EventReader<FighterDespawnEvent>,
    image: Res<ShipImage>,
    score: Res<Score>,
    query: Query<&Transform, With<MyCamera>>,
) {
    if events.is_empty() { return }
    events.clear();
    if score.fighter % 10 != 0 { return }

    let mut rng = rand::thread_rng();
    let Ok(camera_transform) = query.get_single() else { return };
    let camera_y = camera_transform.translation.y;
    let die_x = Uniform::from(-GRID_SIZE * 18.0..GRID_SIZE * 18.0);
    let die_timer = Uniform::from(TIMER_RANGE);
    let translation = Vec3::new(
        die_x.sample(&mut rng),
        camera_y + GRID_SIZE * 13.0,
        10.0,
    );
    let (duration, mode) = (
        die_timer.sample(&mut rng),
        TimerMode::Repeating,
    );
    let direction = if rand::Rng::gen_bool(&mut rng, 1.0 / 1.0) { DIRECTION } else { -DIRECTION };
    // debug!("spawn");
    commands.spawn((
        Sprite::from_image(image.clone()),
        Transform {
            translation,
            rotation: Quat::from_rotation_z(DEGREES.to_radians()),
            scale: SCALE,
        },
        Torpedo { size: SIZE, hp: HP, shoot_timer: Timer::from_seconds(duration, mode) },
        Velocity(direction * SPEED),
    ));
}

fn change_direction(
    mut query: Query<(&mut Velocity, &Transform), With<Torpedo>>,
) {
    for (mut velocity, transform) in &mut query {
        let left_window_collision =
        WINDOW_SIZE.x / 2.0 < transform.translation.x + SIZE.x / 4.0;
        let right_window_collision =
        -WINDOW_SIZE.x / 2.0 > transform.translation.x - SIZE.x / 4.0;

        if left_window_collision || right_window_collision {
            // trace!("change_direction");
            velocity.x = -velocity.x;
        }
    }
}

pub fn damage(
    mut events: EventReader<TorpedoDamageEvent>,
    mut query: Query<(Entity, &mut Torpedo), With<Torpedo>>,
) {
    for event in events.read() {
        let damaged_entity = event.0;

        for (entity, mut torpedo) in &mut query {
            if damaged_entity == entity {
                // debug!("damage");
                torpedo.hp -= 1;
                // trace!("torpedo.hp: {}", torpedo.hp);
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    mut events: EventWriter<TorpedoDespawnEvent>,
    mut score: ResMut<Score>,
    query: Query<(Entity, &Torpedo, &Transform), With<Torpedo>>,
) {
    for (entity, torpedo, transform) in &query {
        if torpedo.hp <= 0 {
            // debug!("despawn");
            events.send(TorpedoDespawnEvent(transform.translation.xy()));
            // trace!("send TorpedoDespawnEvent");
            score.torpedo += 1;
            // trace!("score.torpedo: {}", score.torpedo);
            commands.entity(entity).despawn();
        }
    }
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Torpedo>>,
) {
    // debug!("all_spawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                spawn,
                change_direction,
                damage,
                despawn,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), (
                all_despawn,
            ))
        ;
    }
}

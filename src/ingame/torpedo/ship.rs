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
use crate::ingame::torpedo::{
    TorpedoDamageEvent,
    TorpedoDespawnEvent,
    Torpedo,
};
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "bevy-2dshooting-game/torpedo-ship.png";
const SIZE: Vec2 = Vec2::splat(32.0);
const HP: usize = 3;
const SCORE: usize = 30;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
const SPEED: f32 = 128.0;
const MAX_COUNT: usize = 1;
const TIMER_RANGE: Range<f32> = 1.5..2.0;

#[derive(Resource, Deref)]
struct ShipImage(Handle<Image>);

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct ShipCount(usize);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(ShipImage(handle));
}

fn spawn(
    mut commands: Commands,
    mut count: ResMut<ShipCount>,
    image: Res<ShipImage>,
    score: Res<Score>,
    query: Query<&Transform, With<MyCamera>>,
) {
    if **score == 0 || **score % 100 != 0 { return }
    if **count >= MAX_COUNT { return }

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
    **count += 1;
    // trace!("count: {}", **count);
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
    mut count: ResMut<ShipCount>,
    query: Query<(Entity, &Torpedo, &Transform), With<Torpedo>>,
) {
    for (entity, torpedo, transform) in &query {
        if torpedo.hp <= 0 {
            // debug!("despawn");
            events.send(TorpedoDespawnEvent(transform.translation.xy()));
            // trace!("send TorpedoDespawnEvent");
            **score += SCORE;
            // trace!("score: {}", **score);
            **count -= 1;
            // trace!("count: {}", **count);
            commands.entity(entity).despawn();
        }
    }
}

fn reset_count(mut count: ResMut<ShipCount>) {
    // debug!("reset_count");
    **count = 0;
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
            .insert_resource(ShipCount(0))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                spawn,
                change_direction,
                damage,
                despawn,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), (
                reset_count,
                all_despawn,
            ))
        ;
    }
}

use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::ops::Range;

use crate::{
    WINDOW_SIZE,
    AppState,
    Score,
    MyCamera,
};
use crate::ingame::{
    GRID_SIZE,
    TORPEDO_SIZE as SIZE,
    TorpedoDamageEvent,
    TorpedoShip,
};

const PATH_IMAGE: &str = "bevy-2dshooting-game/torpedo-ship.png";
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
const SPEED: f32 = 128.0;
const MAX_COUNT: usize = 1;
const TIMER_RANGE: Range<f32> = 1.4..1.6;

#[derive(Resource, Deref)]
struct ShipImage(Handle<Image>);

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct ShipCount(usize);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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
    // println!("torpedo.ship: spawn");
    if **score == 0 || **score % 10 != 0 { return }
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
    // ship
    commands.spawn((
        Sprite::from_image(image.clone()),
        Transform {
            translation,
            rotation: Quat::from_rotation_z(DEGREES.to_radians()),
            scale: SCALE,
        },
        TorpedoShip { _shoot_timer: Timer::from_seconds(duration, mode) },
        Velocity(direction * SPEED),
    ));
    **count += 1;
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<TorpedoShip>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("torpedo.ship: apply_velocity");
    for (mut transform, velocity) in &mut query {
        // movement
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

fn change_direction(
    mut query: Query<(&mut Velocity, &Transform), With<TorpedoShip>>,
) {
    // println!("torpedo.ship: change_direction");
    for (mut velocity, transform) in &mut query {
        let left_window_collision =
        WINDOW_SIZE.x / 2.0 < transform.translation.x + SIZE.x / 4.0;
        let right_window_collision =
        -WINDOW_SIZE.x / 2.0 > transform.translation.x - SIZE.x / 4.0;

        if left_window_collision || right_window_collision {
            velocity.x = -velocity.x;
        }
    }
}

pub fn damage(
    mut commands: Commands,
    mut torpedo_damage_events: EventReader<TorpedoDamageEvent>,
    mut count: ResMut<ShipCount>,
    mut score: ResMut<Score>,
) {
    // println!("torpedo.ship: damage");
    for damage in torpedo_damage_events.read() {
        let (entity, _vec2) = (damage.0, damage.1);
        // decrement torpedo count
        **count -= 1;
        // increment score
        **score += 1;
        // despawn ship
        commands.entity(entity).despawn();
    }
}

fn reset_count(mut count: ResMut<ShipCount>) {
    // println!("torpedo.ship: reset_count");
    **count = 0;
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<TorpedoShip>>,
) {
    // println!("torpedo.ship: despawn");
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
                apply_velocity,
                change_direction,
                // damage, // moved ingame/player/bullet.rs
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), (
                reset_count,
                despawn,
            ))
        ;
    }
}
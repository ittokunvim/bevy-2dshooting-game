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
    ENEMY_SIZE as SIZE,
    EnemyDamageEvent,
    EnemyShip,
};
use crate::ingame::enemy::ShipDespawnEvent;

const PATH_IMAGE_ENEMY_SHIP: &str = "bevy-2dshooting-game/enemy-ship.png";
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(1.0, 0.0);
const SPEED: f32 = 256.0;
const MAX_COUNT: usize = 4;
const TIMER_RANGE: Range<f32> = 0.4..0.6;

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
    // println!("enemy.ship: setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_ENEMY_SHIP);
    commands.insert_resource(ShipImage(handle));
}

fn spawn(
    mut commands: Commands,
    mut count: ResMut<ShipCount>,
    image: Res<ShipImage>,
    query: Query<&Transform, With<MyCamera>>,
) {
    // println!("enemy.ship: spawn");
    if **count >= MAX_COUNT { return }

    let mut rng = rand::thread_rng();
    let Ok(camera_transform) = query.get_single() else { return };
    let camera_y = camera_transform.translation.y;
    let die_x = Uniform::from(-GRID_SIZE * 18.0..GRID_SIZE * 18.0);
    let die_y = Uniform::from(camera_y + GRID_SIZE * 10.0..camera_y + GRID_SIZE * 12.0);
    let die_timer = Uniform::from(TIMER_RANGE);
    let translation = Vec3::new(
        die_x.sample(&mut rng),
        die_y.sample(&mut rng),
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
        EnemyShip { shoot_timer: Timer::from_seconds(duration, mode) },
        Velocity(direction * SPEED),
    ));
    **count += 1;
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<EnemyShip>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("enemy.ship: apply_velocity");
    for (mut transform, velocity) in &mut query {
        // movement
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

fn change_direction(
    mut query: Query<(&mut Velocity, &Transform), With<EnemyShip>>,
) {
    // println!("enemy.ship: change_direction");
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
    mut enemy_damage_events: EventReader<EnemyDamageEvent>,
    mut ship_despawn_events: EventWriter<ShipDespawnEvent>,
    mut count: ResMut<ShipCount>,
    mut score: ResMut<Score>,
) {
    // println!("enemy.ship: damage");
    for damage in enemy_damage_events.read() {
        let (entity, vec2) = (damage.0, damage.1);
        // send despawn event
        ship_despawn_events.send(ShipDespawnEvent(vec2));
        // decrement enemy count
        **count -= 1;
        // increment score
        **score += 1;
        // despawn ship
        commands.entity(entity).despawn();
    }
}

fn reset_count(mut count: ResMut<ShipCount>) {
    **count = 0;
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<EnemyShip>>,
) {
    // println!("enemy.ship: despawn");
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

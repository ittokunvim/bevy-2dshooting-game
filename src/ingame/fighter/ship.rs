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
use crate::ingame::fighter::{
    FighterDamageEvent,
    ShipDespawnEvent,
    Fighter,
};
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "bevy-2dshooting-game/fighter-ship.png";
const SIZE: Vec2 = Vec2::splat(32.0);
const HP: usize = 1;
const SCORE: usize = 10;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(1.0, -0.05);
const SPEED: f32 = 256.0;
const MAX_COUNT: usize = 4;
const TIMER_RANGE: Range<f32> = 0.4..0.6;

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
    query: Query<&Transform, With<MyCamera>>,
) {
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
        Fighter { size: SIZE, hp: HP, shoot_timer: Timer::from_seconds(duration, mode) },
        Velocity(direction * SPEED),
    ));
    **count += 1;
}

fn change_direction(
    mut query: Query<(&Fighter, &mut Velocity, &Transform), With<Fighter>>,
) {
    for (fighter, mut velocity, transform) in &mut query {
        let left_window_collision =
        WINDOW_SIZE.x / 2.0 < transform.translation.x + fighter.size.x / 4.0;
        let right_window_collision =
        -WINDOW_SIZE.x / 2.0 > transform.translation.x - fighter.size.x / 4.0;

        if left_window_collision || right_window_collision {
            velocity.x = -velocity.x;
        }
    }
}

fn damage(
    mut events: EventReader<FighterDamageEvent>,
    mut query: Query<(Entity, &mut Fighter), With<Fighter>>,
) {
    for event in events.read() {
        let damage_entity = event.0;

        for (entity, mut fighter) in &mut query {
            if damage_entity == entity {
                fighter.hp -= 1;
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    mut events: EventWriter<ShipDespawnEvent>,
    mut score: ResMut<Score>,
    mut count: ResMut<ShipCount>,
    query: Query<(Entity, &Fighter, &Transform), With<Fighter>>,
) {
    for (entity, fighter, transform) in &query {
        if fighter.hp <= 0 {
            events.send(ShipDespawnEvent(transform.translation.xy()));
            **score += SCORE;
            **count -= 1;
            commands.entity(entity).despawn();
        }
    }
}

fn reset_count(mut count: ResMut<ShipCount>) {
    **count = 0;
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Fighter>>,
) {
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

use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::{
    WINDOW_SIZE,
    AppState,
};
use crate::ingame::{
    GRID_SIZE,
    ENEMY_SIZE as SIZE,
    EnemyShip,
    PlayerBulletHitEvent,
};
use crate::ingame::enemy::EnemyDespawnEvent;

const PATH_IMAGE_ENEMY_SHIP: &str = "bevy-2dshooting-game/enemy-ship.png";
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const DIRECTION: Vec2 = Vec2::new(-1.0, 0.0);
const SPEED: f32 = 256.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("enemy.ship: setup");
    let image = asset_server.load(PATH_IMAGE_ENEMY_SHIP);
    let mut rng = rand::thread_rng();
    let die_x = Uniform::from(-GRID_SIZE * 18.0..GRID_SIZE * 18.0);
    let die_y = Uniform::from(GRID_SIZE * 10.0..GRID_SIZE * 12.0);
    let translation = Vec3::new(
        die_x.sample(&mut rng),
        die_y.sample(&mut rng),
        10.0,
    );
    commands.spawn((
        Sprite::from_image(image),
        Transform {
            translation,
            rotation: Quat::from_rotation_z(DEGREES.to_radians()),
            scale: SCALE,
        },
        EnemyShip,
        Velocity(DIRECTION * SPEED),
    ));
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<EnemyShip>>,
    time_step: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        // move enemy
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

fn change_direction(
    mut query: Query<(&mut Velocity, &Transform), With<EnemyShip>>,
) {
    for (mut velocity, transform) in &mut query {
        let left_window_collision =
        WINDOW_SIZE.x / 2.0 < transform.translation.x + SIZE.x / 4.0;
        let right_window_collision =
        -WINDOW_SIZE.x / 2.0 > transform.translation.x - SIZE.x / 4.0;

        if left_window_collision || right_window_collision {
            // println!("enemy.ship: change direction");
            velocity.x = -velocity.x;
        }
    }
}

fn despawn(
    mut commands: Commands,
    mut player_bullet_hit_events: EventReader<PlayerBulletHitEvent>,
    mut enemy_despawn_events: EventWriter<EnemyDespawnEvent>,
    query: Query<Entity, With<EnemyShip>>,
) {
    if player_bullet_hit_events.is_empty() { return }
    player_bullet_hit_events.clear();

    for entity in &query {
        enemy_despawn_events.send_default();
        // println!("enemy.ship: despawn");
        commands.entity(entity).despawn();
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                apply_velocity,
                change_direction,
                despawn,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

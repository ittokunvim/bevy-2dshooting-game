use bevy::prelude::*;

mod enemy;
mod player;
mod camera;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;
const CAMERA_SPEED: f32 = 0.2;
const PLAYER_SIZE: Vec2 = Vec2::splat(32.0);
const PLAYER_LIFE: usize = 8;
const ENEMY_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Resource, Deref, DerefMut)]
struct PlayerLife(usize);

#[derive(Event, Default)]
struct PlayerDamageEvent;

#[derive(Event)]
struct EnemyDamageEvent(Entity, Vec2);

#[derive(Component)]
struct PlayerShip;

#[derive(Component)]
struct EnemyShip {
    shoot_timer: Timer,
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerLife(PLAYER_LIFE))
            .add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

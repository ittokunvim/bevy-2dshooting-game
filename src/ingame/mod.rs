use bevy::prelude::*;

mod enemies;
mod player;
mod camera;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;
const CAMERA_SPEED: f32 = 0.2;
const PLAYER_SIZE: Vec2 = Vec2::splat(32.0);
const PLAYER_LIFE: usize = 8;
const FIGHTER_SIZE: Vec2 = Vec2::splat(32.0);
const TORPEDO_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Resource, Deref, DerefMut)]
struct PlayerLife(usize);

#[derive(Event, Default)]
struct PlayerDamageEvent;

#[derive(Event)]
struct FighterDamageEvent(Entity, Vec2);

#[derive(Event)]
struct TorpedoDamageEvent(Entity, Vec2);

#[derive(Component)]
struct PlayerShip;

#[derive(Component)]
struct FighterShip {
    shoot_timer: Timer,
}

#[derive(Component)]
struct TorpedoShip {
    _shoot_timer: Timer,
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerLife(PLAYER_LIFE))
            .add_event::<PlayerDamageEvent>()
            .add_event::<FighterDamageEvent>()
            .add_event::<TorpedoDamageEvent>()
            .add_plugins(enemies::EnemiesPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

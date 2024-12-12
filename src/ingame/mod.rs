use bevy::prelude::*;

mod enemy;
mod player;

const GRID_SIZE: f32 = 16.0;
const PATH_IMAGE_PLAYER_SHIP: &str = "bevy-2dshooting-game/player-ship.png";
const PATH_IMAGE_PLAYER_BULLET: &str = "bevy-2dshooting-game/player-bullet.png";
const PATH_IMAGE_ENEMY_SHIP: &str = "bevy-2dshooting-game/enemy-ship.png";
const PATH_IMAGE_ENEMY_BULLET: &str = "bevy-2dshooting-game/enemy-bullet.png";
const PATH_SOUND_SHOOT: &str = "bevy-2dshooting-game/shoot.ogg";

const PLAYER_SIZE: Vec2 = Vec2::splat(64.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Component)]
struct PlayerShip;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct EnemyShip;

#[derive(Component)]
struct EnemyBullet;

#[derive(Event, Default)]
struct PlayerBulletHitEvent;

#[derive(Event, Default)]
struct EnemyBulletHitEvent;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerBulletHitEvent>()
            .add_event::<EnemyBulletHitEvent>()
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
        ;
    }
}

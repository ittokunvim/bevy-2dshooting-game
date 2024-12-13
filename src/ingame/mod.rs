use bevy::prelude::*;

mod enemy;
mod player;

const GRID_SIZE: f32 = 16.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(32.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Component)]
struct PlayerShip;

#[derive(Component)]
struct EnemyShip;

#[derive(Event)]
struct PlayerBulletHitEvent(Entity, Vec2);

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

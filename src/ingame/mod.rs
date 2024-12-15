use bevy::prelude::*;

mod enemy;
mod player;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;
const PLAYER_SIZE: Vec2 = Vec2::splat(32.0);
const ENEMY_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct PlayerShip;

#[derive(Component)]
struct EnemyShip {
    shoot_timer: Timer,
}

#[derive(Event, Default)]
struct PlayerDamageEvent;

#[derive(Event)]
struct EnemyDamageEvent(Entity, Vec2);

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

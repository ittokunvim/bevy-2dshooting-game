use bevy::prelude::*;

mod enemies;
mod fighter;
mod player;
mod camera;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;
const CAMERA_SPEED: f32 = 0.2;
const FIGHTER_SIZE: Vec2 = Vec2::splat(32.0);
const TORPEDO_SIZE: Vec2 = Vec2::splat(32.0);

#[derive(Event, Default)]
struct PlayerDamageEvent;

#[derive(Event)]
struct FighterDamageEvent(Entity);

#[derive(Event)]
struct TorpedoDamageEvent(Entity);

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerDamageEvent>()
            .add_event::<FighterDamageEvent>()
            .add_event::<TorpedoDamageEvent>()
            .add_plugins(enemies::EnemiesPlugin)
            .add_plugins(fighter::FighterPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

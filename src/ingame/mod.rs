use bevy::prelude::*;

mod fighter;
mod player;
mod torpedo;
mod camera;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;
const CAMERA_SPEED: f32 = 0.2;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fighter::FighterPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(torpedo::TorpedoPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

use bevy::prelude::*;

pub mod fighter;
pub mod torpedo;

mod utils;
mod player;
mod camera;
mod scoreboard;

const GRID_SIZE: f32 = 16.0;

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fighter::FighterPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(torpedo::TorpedoPlugin)
            .add_plugins(utils::UtilsPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}

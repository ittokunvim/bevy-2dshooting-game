use bevy::prelude::*;

pub mod fighter;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fighter::FighterPlugin)
        ;
    }
}

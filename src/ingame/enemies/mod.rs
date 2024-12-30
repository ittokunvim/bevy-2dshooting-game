use bevy::prelude::*;

use crate::AppState;

pub mod fighter;
pub mod torpedo;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fighter::FighterPlugin)
            .add_plugins(torpedo::TorpedoPlugin)
            .add_systems(Update, (
                crate::ingame::enemies::fighter::bullet::check_for_hit,
                crate::ingame::enemies::torpedo::bullet::check_for_hit,
                crate::ingame::player::ship::damage_life,
                crate::ingame::player::ship::damage_animation,
                crate::ingame::player::ship::damage_despawn,
             ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}

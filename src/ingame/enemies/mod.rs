use bevy::prelude::*;

pub mod fighter;
pub mod torpedo;

mod bullet;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(fighter::FighterPlugin)
            .add_plugins(torpedo::TorpedoPlugin)
        ;
    }
}

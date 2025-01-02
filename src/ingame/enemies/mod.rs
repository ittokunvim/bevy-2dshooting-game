use bevy::prelude::*;

pub mod torpedo;

mod bullet;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(torpedo::TorpedoPlugin)
        ;
    }
}

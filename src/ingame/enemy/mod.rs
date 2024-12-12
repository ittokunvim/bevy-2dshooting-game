use bevy::prelude::*;

mod bullet;
mod ship;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

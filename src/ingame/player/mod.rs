use bevy::prelude::*;

mod bullet;
mod ship;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

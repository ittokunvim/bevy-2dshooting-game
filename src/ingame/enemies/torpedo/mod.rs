use bevy::prelude::*;

pub mod ship;
pub mod bullet;

#[derive(Component)]
pub struct Torpedo {
    hp: usize,
    shoot_timer: Timer,
}

pub struct TorpedoPlugin;

impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

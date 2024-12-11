use bevy::prelude::*;

mod bullet;
mod ship;

#[derive(Component)]
struct Player {
    first: usize,
    last: usize,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

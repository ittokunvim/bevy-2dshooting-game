use bevy::prelude::*;

mod bullet;
mod ship;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet {
    first: usize,
    last: usize,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

use bevy::prelude::*;

mod ship;
mod bullet;
mod despawn;

#[derive(Event, Default)]
struct EnemyDespawnEvent;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyDespawnEvent>()
            .add_plugins(ship::ShipPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(despawn::DespawnPlugin)
        ;
    }
}

use bevy::prelude::*;

mod ship;
mod bullet;
mod sound;
mod despawn;

#[derive(Event)]
struct ShipDespawnEvent(Vec2);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShipDespawnEvent>()
            .add_plugins(ship::ShipPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(sound::SoundPlugin)
            .add_plugins(despawn::DespawnPlugin)
        ;
    }
}

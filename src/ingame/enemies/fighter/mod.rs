use bevy::prelude::*;

pub mod ship;
mod bullet;
mod sound;
mod despawn;

#[derive(Event)]
pub struct ShipDespawnEvent(Vec2);

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
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

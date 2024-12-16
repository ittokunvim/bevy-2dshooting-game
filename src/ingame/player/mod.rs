use bevy::prelude::*;

pub mod ship;
mod bullet;
mod sound;

#[derive(Event, Default)]
struct ShootEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootEvent>()
            .add_plugins(ship::ShipPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(sound::SoundPlugin)
        ;
    }
}

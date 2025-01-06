use bevy::prelude::*;

pub mod ship;
pub mod bullet;
mod despawn;
mod sound;

#[derive(Event)]
pub struct TorpedoDamageEvent(pub Entity);

#[derive(Event)]
pub struct TorpedoDespawnEvent(Vec2);

#[derive(Event, Default)]
struct TorpedoSpawnEvent;

#[derive(Component)]
pub struct Torpedo {
    pub size: Vec2,
    hp: usize,
    shoot_timer: Timer,
}

pub struct TorpedoPlugin;

impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TorpedoDamageEvent>()
            .add_event::<TorpedoDespawnEvent>()
            .add_event::<TorpedoSpawnEvent>()
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
            .add_plugins(despawn::DespawnPlugin)
            .add_plugins(sound::SoundPlugin)
        ;
    }
}

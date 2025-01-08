use bevy::prelude::*;

mod bullet;
mod despawn;
mod ship;
mod sound;

pub const PATH_IMAGE_TORPEDO: &str = "bevy-2dshooting-game/torpedo-ship.png";

#[derive(Event)]
pub struct TorpedoDamageEvent(pub Entity);

#[derive(Event)]
pub struct TorpedoDespawnEvent(Vec2);

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
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(despawn::DespawnPlugin)
            .add_plugins(ship::ShipPlugin)
            .add_plugins(sound::SoundPlugin)
        ;
    }
}

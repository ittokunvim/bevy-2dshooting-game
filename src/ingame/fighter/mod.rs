use bevy::prelude::*;

pub mod ship;
pub mod bullet;
mod sound;
mod despawn;

#[derive(Event)]
pub struct FighterDamageEvent(pub Entity);

#[derive(Event)]
pub struct ShipDespawnEvent(Vec2);

#[derive(Component)]
pub struct Fighter {
    pub size: Vec2,
    hp: usize,
    shoot_timer: Timer,
}

pub struct FighterPlugin;

impl Plugin for FighterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FighterDamageEvent>()
            .add_event::<ShipDespawnEvent>()
            .add_plugins(ship::ShipPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(sound::SoundPlugin)
            .add_plugins(despawn::DespawnPlugin)
        ;
    }
}

use bevy::prelude::*;

mod bullet;
mod despawn;
mod ship;
mod sound;

pub const PATH_IMAGE_FIGHTER: &str = "bevy-2dshooting-game/fighter-ship.png";

#[derive(Event)]
pub struct FighterDamageEvent(pub Entity);

#[derive(Event)]
pub struct FighterDespawnEvent(Vec2);

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
            .add_event::<FighterDespawnEvent>()
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(despawn::DespawnPlugin)
            .add_plugins(ship::ShipPlugin)
            .add_plugins(sound::SoundPlugin)
        ;
    }
}

use bevy::prelude::*;

pub mod ship;
mod bullet;
mod sound;

pub const PLAYER_HP: usize = 8;

#[derive(Event, Default)]
struct ShootEvent;

#[derive(Component)]
pub struct Player {
    pub hp: usize,
    pub size: Vec2,
    pub bullets: usize,
}

impl Player {
    fn new(hp: usize, size: Vec2, bullets: usize) -> Self {
        Self { hp, size, bullets, }
    }
}

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

use bevy::prelude::*;

pub mod ship;
pub mod bullet;

#[derive(Event)]
pub struct TorpedoDamageEvent(pub Entity);

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
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

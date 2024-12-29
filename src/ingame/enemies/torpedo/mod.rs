use bevy::prelude::*;

pub mod ship;

pub struct TorpedoPlugin;

impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

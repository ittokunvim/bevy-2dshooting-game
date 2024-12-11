use bevy::prelude::*;

mod ship;

#[derive(Component)]
struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ship::ShipPlugin)
        ;
    }
}

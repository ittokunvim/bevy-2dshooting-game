use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    PATH_IMAGE_ENEMY_SHIP,
};
use crate::enemy::Enemy;

const SCALE: Vec3 = Vec3::splat(1.0);
const TRANSLATION: Vec3 = Vec3::new(0.0, GRID_SIZE * 12.0, 99.0);
const DEGREES: f32 = 180.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("enemy: setup");
    let image = asset_server.load(PATH_IMAGE_ENEMY_SHIP);

    commands.spawn((
        Sprite::from_image(image),
        Transform {
            translation: TRANSLATION,
            rotation: Quat::from_rotation_z(DEGREES.to_radians()),
            scale: SCALE,
        },
        Enemy,
    ));
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
        ;
    }
}

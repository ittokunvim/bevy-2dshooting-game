use bevy::prelude::*;

use crate::{
    AppState,
    MyCamera,
};
use crate::ingame::CAMERA_SPEED as SPEED;

fn scrollup(
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    // println!("camera: scrollup");
    let Ok(mut transform) = query.get_single_mut() else { return };

    transform.translation.y += SPEED;
}

fn reset_position(
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    // println!("camera: reset_position");
    let Ok(mut transform) = query.get_single_mut() else { return };

    transform.translation = Vec3::ZERO;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, scrollup.run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Gameover), reset_position)
        ;
    }
}

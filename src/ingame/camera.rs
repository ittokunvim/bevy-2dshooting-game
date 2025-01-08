use bevy::prelude::*;

use crate::{
    AppState,
    MyCamera,
};

pub const SPEED: f32 = 0.2;

fn scrollup(
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };

    transform.translation.y += SPEED;
    // trace!("camera y: {}", transform.translation.y);
}

fn reset_position(
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    // debug!("reset_position");
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

use bevy::prelude::*;

use crate::{
    AppState,
    MyCamera,
};
use crate::ingame::CAMERA_SPEED as SPEED;

fn scrollup(
    mut query: Query<&mut Transform, With<MyCamera>>,
) {
    let Ok(mut transform) = query.get_single_mut() else { return };

    transform.translation.y += SPEED;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, scrollup.run_if(in_state(AppState::Ingame)))
        ;
    }
}

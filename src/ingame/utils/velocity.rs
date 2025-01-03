use bevy::prelude::*;

use crate::AppState;
use crate::ingame::CAMERA_SPEED;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time_step: Res<Time<Fixed>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
        transform.translation.y += CAMERA_SPEED;
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_velocity.run_if(in_state(AppState::Ingame)))
        ;
    }
}

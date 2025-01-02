use bevy::prelude::*;

pub mod animation_config;
pub mod velocity;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(animation_config::AnimationConfigPlugin)
            .add_plugins(velocity::VelocityPlugin)
        ;
    }
}

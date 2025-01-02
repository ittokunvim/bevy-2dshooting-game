use bevy::prelude::*;

pub mod animation_config;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(animation_config::AnimationConfigPlugin)
        ;
    }
}

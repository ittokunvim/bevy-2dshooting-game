use bevy::prelude::*;

pub mod prelude;

mod animation_config;
mod bullet;
mod velocity;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(animation_config::AnimationConfigPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(velocity::VelocityPlugin)
        ;
    }
}

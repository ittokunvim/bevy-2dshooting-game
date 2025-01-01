use bevy::prelude::*;

use crate::AppState;
use crate::ingame::{
    GRID_SIZE,
    TorpedoShip,
};
use crate::ingame::player::Player;
use crate::ingame::enemies::bullet::{
    AnimationConfig,
    Velocity,
    Bullet,
};

const PATH_IMAGE: &str = "bevy-2dshooting-game/torpedo-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(11, 32);
const COLUMN: u32 = 3;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(1.5);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(16.5, 48.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut torpedo_query: Query<(&mut TorpedoShip, &Transform), (With<TorpedoShip>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<TorpedoShip>)>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };

    for (mut torpedo, torpedo_transform) in &mut torpedo_query {
        if !torpedo.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let torpedo_xy = torpedo_transform.translation.xy();
        let translation = Vec3::new(
            torpedo_xy.x, 
            torpedo_xy.y - GRID_SIZE * 2.0, 
            99.0,
        );
        let player_xy = player_transform.translation.xy();
        let delta_xy = (player_xy - translation.xy()).normalize();
        let degrees = delta_xy.y.atan2(delta_xy.x).to_degrees() - 90.0;

        let animation_config = AnimationConfig::new(0, 2, FPS);
        let velocity = Velocity::new(delta_xy * SPEED);
        let bullet = Bullet::new(
            SIZE,
            bullet_image.clone(),
            texture_atlas_layout.clone(),
            animation_config.first_sprite_index,
            translation,
            degrees,
            SCALE,
        );
        // bullet
        commands.spawn((bullet, animation_config, velocity));
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, shoot.run_if(in_state(AppState::Ingame)))
        ;
    }
}

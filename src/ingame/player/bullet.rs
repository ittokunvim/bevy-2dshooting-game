use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GRID_SIZE;
use crate::ingame::player::{
    ShootEvent,
    Player,
};
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "ittoku-2dshooting-game/player-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const DIRECTION: Vec2 = Vec2::new(0.0, 1.0);
const SPEED: f32 = 512.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::splat(32.0);
const DEGREES: f32 = 0.0;
const SCALE: Vec3 = Vec3::splat(1.0);
const KEYCODE: KeyCode = KeyCode::Space;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn event(
    mut commands: Commands,
    mut player_query: Query<&mut Player, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) -> Result {
    if !keyboard_input.just_pressed(KEYCODE) { return Ok(()) }

    let mut player = player_query.single_mut()?;

    if player.bullets <= 0 { return Ok(()) }
    // debug!("event");
    commands.trigger(ShootEvent);
    player.bullets -= 1;
    // trace!("player.bullets: {}", player.bullets);

    Ok(())
}

fn shoot(
    _shot: On<ShootEvent>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<&Transform, With<Player>>,
    bullet_image: Res<BulletImage>,
) -> Result {
    let transform = player_query.single()?;
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let translation = Vec3::new(
        transform.translation.x, 
        transform.translation.y + GRID_SIZE * 2.0, 
        99.0,
    );

    let animation_config = AnimationConfig::new(AnimationName::Bullet, 0, 3, FPS);
    let velocity = Velocity(DIRECTION * SPEED);
    let bullet = Bullet::new(
        Shooter::Player, 
        SIZE, 
        bullet_image.clone(), 
        texture_atlas_layout.clone(), 
        animation_config.first_sprite_index, 
        translation, 
        DEGREES, 
        SCALE,
    );
    // debug!("shoot");
    commands.spawn((bullet, animation_config, velocity));

    Ok(())
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                event,
            ).run_if(in_state(AppState::Ingame)))
            .add_observer(shoot)
        ;
    }
}

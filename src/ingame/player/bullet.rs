use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GRID_SIZE;
use crate::ingame::player::{
    ShootEvent,
    Player,
};
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "bevy-2dshooting-game/player-bullet.png";
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
    mut events: EventWriter<ShootEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KEYCODE) { return }
    // debug!("event");
    events.send_default();
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut events: EventReader<ShootEvent>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    bullet_image: Res<BulletImage>,
) {
    let Ok((mut player, transform)) = player_query.get_single_mut() else { return };

    if events.is_empty() { return }
    events.clear();
    if player.bullets <= 0 { return }

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
    player.bullets -= 1;
    // trace!("player.bullets: {}", player.bullets);
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                event,
                shoot,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

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
const MAX_COUNT: usize = 2;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Resource, Deref, DerefMut, Debug)]
struct Remaining(usize);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn event(
    mut events: EventWriter<ShootEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KEYCODE) { return }
    // send shoot event
    events.send_default();
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut events: EventReader<ShootEvent>,
    mut remaining: ResMut<Remaining>,
    bullet_image: Res<BulletImage>,
    player_query: Query<&Transform, With<Player>>,
) {
    if events.is_empty() || **remaining <= 0 { return }
    events.clear();

    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let Ok(player_transform) = player_query.get_single() else { return };
    let translation = Vec3::new(
        player_transform.translation.x, 
        player_transform.translation.y + GRID_SIZE * 2.0, 
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
    // bullet
    commands.spawn((bullet, animation_config, velocity));
    // increase remaining bullet
    **remaining -= 1;
}

fn reset_remaining(mut remaining: ResMut<Remaining>) {
    **remaining = MAX_COUNT;
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Remaining(MAX_COUNT))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                event,
                shoot,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), reset_remaining)
        ;
    }
}

use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
    MyCamera,
};
use crate::ingame::{
    GRID_SIZE,
    CAMERA_SPEED,
};
use crate::ingame::player::{
    PlayerDamageEvent,
    Player,
};
use crate::ingame::utils::animation_config::{
    AnimationConfig,
    AnimationName,
};

const PATH_IMAGE: &str = "bevy-2dshooting-game/player-ship.png";
const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const HP: usize = 8;
const SIZE: Vec2 = Vec2::splat(32.0);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const TRANSLATION: Vec3 = Vec3::new(0.0, GRID_SIZE * -12.0, 99.0);
const SCALE: Vec3 = Vec3::splat(2.0);
const SPEED: f32 = 256.0;

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load(PATH_IMAGE);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationConfig::new(AnimationName::PlayerDamage, 0, 3, 0.0);
    let player = Player { hp: HP, size: SIZE };
    // player ship
    commands.spawn((
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first_sprite_index,
            },
        ),
        Transform {
            translation: TRANSLATION,
            scale: SCALE,
            ..Default::default()
        },
        animation_indices,
        player,
    ));
}

fn movement(
    mut player_query: Query<&mut Transform, (With<Player>, Without<MyCamera>)>,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time<Fixed>>,
) {
    let mut direction = Vec2::ZERO;
    // set direction
    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::ArrowLeft  | KeyCode::KeyA => direction.x -= 1.0,
            KeyCode::ArrowRight | KeyCode::KeyD => direction.x += 1.0,
            KeyCode::ArrowUp    | KeyCode::KeyW => direction.y += 1.0,
            KeyCode::ArrowDown  | KeyCode::KeyS => direction.y -= 1.0,
            _ => {},
        }
    }

    let Ok(mut ship_transform) = player_query.get_single_mut() else { return };
    let ship_xy = ship_transform.translation.xy();
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_xy = camera_transform.translation.xy();
    // set player x position
    let new_player_position_x = ship_xy.x
    + direction.x * SPEED * time_step.delta().as_secs_f32();
    // set player x range movement
    let left_bound = camera_xy.x - WINDOW_SIZE.x / 2.0 + SIZE.x;
    let right_bound = camera_xy.x + WINDOW_SIZE.x / 2.0 - SIZE.x;
    // set player y position
    let new_player_position_y = ship_xy.y
    + direction.y * SPEED * time_step.delta().as_secs_f32();
    // set player y range movement
    let down_bound = camera_xy.y - WINDOW_SIZE.y / 2.0 + SIZE.y;
    let up_bound = camera_xy.y + WINDOW_SIZE.y / 2.0 - SIZE.y;
    // move player
    ship_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    ship_transform.translation.y = new_player_position_y.clamp(down_bound, up_bound);
    ship_transform.translation.y += CAMERA_SPEED;
}

fn damage(
    mut events: EventReader<PlayerDamageEvent>,
    mut query: Query<&mut Player, With<Player>>,
) {
    if events.is_empty() { return }
    events.clear();

    let Ok(mut player) = query.get_single_mut() else { return };

    player.hp -= 1;
}

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Player), With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok((entity, player)) = query.get_single() else { return };

    if player.hp <= 0 {
        // despawn ship
        commands.entity(entity).despawn();
        // moved app state Ingame -> Gameover
        next_state.set(AppState::Gameover);
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                movement,
                damage,
                despawn,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

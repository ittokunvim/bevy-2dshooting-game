use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};
use crate::ingame::{
    GRID_SIZE,
    PLAYER_SIZE as SIZE,
    PLAYER_LIFE as LIFE,
    PlayerLife,
    PlayerShip,
    PlayerDamageEvent,
};

const PATH_IMAGE_PLAYER_SHIP: &str = "bevy-2dshooting-game/player-ship.png";
const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const TRANSLATION: Vec3 = Vec3::new(0.0, GRID_SIZE * -12.0, 99.0);
const SPEED: f32 = 256.0;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // println!("player.ship: setup");
    let texture = asset_server.load(PATH_IMAGE_PLAYER_SHIP);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3, };
    // player ship
    commands.spawn((
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform {
            translation: TRANSLATION,
            scale: SCALE,
            ..Default::default()
        },
        animation_indices,
        PlayerShip,
    ));
}

fn movement(
    mut query: Query<&mut Transform, With<PlayerShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time<Fixed>>,
) {
    // println!("player.ship: movement");
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

    let Ok(mut transform) = query.get_single_mut() else { return };
    // set player x position
    let new_player_position_x = transform.translation.x
        + direction.x * SPEED * time_step.delta().as_secs_f32();
    // set player x range movement
    let left_bound = -WINDOW_SIZE.x / 2.0 + SIZE.x / 2.0;
    let right_bound = WINDOW_SIZE.x / 2.0 - SIZE.x / 2.0;
    // set player y position
    let new_player_position_y = transform.translation.y
        + direction.y * SPEED * time_step.delta().as_secs_f32();
    // set player y range movement
    let up_bound = -WINDOW_SIZE.y / 2.0 + SIZE.y / 2.0;
    let down_bound = WINDOW_SIZE.y / 2.0 - SIZE.y / 2.0;
    // move player
    transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    transform.translation.y = new_player_position_y.clamp(up_bound, down_bound);
}

pub fn damage_life(
    mut events: EventReader<PlayerDamageEvent>,
    mut life: ResMut<PlayerLife>,
) {
    // println!("player.ship: damage_life");
    if events.is_empty() { return }
    events.clear();
    // reduce player life
    **life -= 1;
}

pub fn damage_animation(
    mut query: Query<(&AnimationIndices, &mut Sprite), With<PlayerShip>>,
    mut events: EventReader<PlayerDamageEvent>,
    life: Res<PlayerLife>,
) {
    // println!("player.ship: damage_animation");
    if events.is_empty() { return }
    events.clear();

    let Ok((indices, mut sprite)) = query.get_single_mut() else { return };
    // do animation
    if **life == 6 || **life == 4 || **life == 2 {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == indices.last 
                { indices.first } else { atlas.index + 1 }
        }
    }
}

pub fn damage_despawn(
    mut commands: Commands,
    query: Query<Entity, With<PlayerShip>>,
    life: Res<PlayerLife>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // println!("player.ship: damage_despawn");
    let Ok(entity) = query.get_single() else { return };

    if **life <= 0 {
        // despawn ship
        commands.entity(entity).despawn();
        // moved app state Ingame -> Gameover
        next_state.set(AppState::Gameover);
    }
}

fn reset_life(mut life: ResMut<PlayerLife>) {
    // println!("player.ship: reset_life");
    **life = LIFE;
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<PlayerShip>>,
) {
    // println!("player.ship: despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                movement,
                // damage_life,      // moved ingame/enemy/bullet.rs
                // damage_animation, // moved ingame/enemy/bullet.rs
                // damage_despawn,   // moved ingame/enemy/bullet.rs
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Gameover), reset_life)
            .add_systems(OnExit(AppState::Ingame), despawn)
        ;
    }
}

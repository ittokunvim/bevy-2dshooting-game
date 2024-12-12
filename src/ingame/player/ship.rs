use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};
use crate::ingame::{
    GRID_SIZE,
    PATH_IMAGE_PLAYER_SHIP,
    PLAYER_SIZE as SIZE,
    PlayerShip,
    EnemyBulletHitEvent,
};

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const TRANSLATION: Vec3 = Vec3::new(0.0, GRID_SIZE * -12.0, 99.0);
const SPEED: f32 = 256.0;

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    println!("player.ship: setup");
    let texture = asset_server.load(PATH_IMAGE_PLAYER_SHIP);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3, };

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

    let mut transform = query.single_mut();
    // set player x position
    let new_player_position_x = transform.translation.x
        + direction.x * SPEED * time_step.delta().as_secs_f32();
    let left_bound = -WINDOW_SIZE.x / 2.0 + SIZE.x / 2.0;
    let right_bound = WINDOW_SIZE.x / 2.0 - SIZE.x / 2.0;
    // set player y position
    let new_player_position_y = transform.translation.y
        + direction.y * SPEED * time_step.delta().as_secs_f32();
    let up_bound = -WINDOW_SIZE.y / 2.0 + SIZE.y / 2.0;
    let down_bound = WINDOW_SIZE.y / 2.0 - SIZE.y / 2.0;
    // movement player
    transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    transform.translation.y = new_player_position_y.clamp(up_bound, down_bound);
}

fn damege(
    mut query: Query<(&AnimationIndices, &mut Sprite), With<PlayerShip>>,
    mut events: EventReader<EnemyBulletHitEvent>,
) {
    if events.is_empty() { return }
    events.clear();

    let Ok((indices, mut sprite)) = query.get_single_mut() else { return };
    // println!("player.ship: damege");
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = if atlas.index == indices.last
            { indices.first } else { atlas.index + 1 }
    }
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                movement,
                damege,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

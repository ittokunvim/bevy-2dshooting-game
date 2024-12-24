use bevy::prelude::*;

use crate::{
    PATH_FONT,
    AppState,
    Score,
    MyCamera,
};

const GAMEOVER_TEXT: &str = "ゲームオーバー";
const GAMEOVER_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "スコア: ";
const RETRY_TEXT: &str = "リトライ: Key[R]";
const BACKTOTITLE_TEXT: &str = "タイトルに戻る: Key[B]";
const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_SIZE: f32 = 16.0;
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
struct Gameover;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<MyCamera>>,
    score: Res<Score>,
) {
    // println!("gameover: setup");
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = camera_transform.translation.y;
    // game over
    let (x, y, z) = (
        0.0,
        camera_y + TEXT_PADDING * 1.5,
        0.0,
    );
    commands.spawn((
        Text2d::new(GAMEOVER_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: GAMEOVER_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Transform::from_xyz(x, y, z),
        Gameover,
    ));
    // score
    let (x, y, z) = (
        0.0,
        camera_y + TEXT_PADDING * 0.5,
        0.0,
    );
    commands.spawn((
        Text2d::new(format!("{}{}", SCORE_TEXT, **score)),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Transform::from_xyz(x, y, z),
        Gameover,
    ));
    // retry
    let (x, y, z) = (
        0.0,
        camera_y - TEXT_PADDING * 0.5,
        0.0,
    );
    commands.spawn((
        Text2d::new(RETRY_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Transform::from_xyz(x, y, z),
        Gameover,
    ));
    // back to title
    let (x, y, z) = (
        0.0,
        camera_y - TEXT_PADDING * 1.5,
        0.0,
    );
    commands.spawn((
        Text2d::new(BACKTOTITLE_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Transform::from_xyz(x, y, z),
        Gameover,
    ));
    // board
    let (x, y, z) = (
        0.0,
        camera_y,
        -10.0,
    );
    commands.spawn((
        Sprite {
            color: BOARD_COLOR,
            custom_size: Some(BOARD_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(x, y, z),
        Gameover,
    ));
}

fn update(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // println!("gameover: update");
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => next_state.set(AppState::Ingame),
            KeyCode::KeyB => next_state.set(AppState::Mainmenu),
            _ => {},
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Gameover>>,
) {
    // println!("gameover: despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}

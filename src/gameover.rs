use bevy::prelude::*;

use crate::{
    PATH_FONT,
    AppState,
    Score,
    MyCamera,
};
use crate::ingame::fighter::PATH_IMAGE_FIGHTER;
use crate::ingame::torpedo::PATH_IMAGE_TORPEDO;

const GAMEOVER_TEXT: &str = "ゲームオーバー";
const GAMEOVER_SIZE: f32 = 20.0;
const SCORE_TEXT: &str = "スコア: ";
const FIGHTER_SCALE: Vec3 = Vec3::splat(0.75);
const TORPEDO_SCALE: Vec3 = Vec3::new(0.5, 0.75, 0.0);
const RETRY_TEXT: &str = "リトライ: Key[R]";
const BACKTOTITLE_TEXT: &str = "タイトルに戻る: Key[B]";
const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_SIZE: f32 = 16.0;
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
#[require(Text2d, TextFont, TextColor, Transform)]
struct Gameover;

impl Gameover {
    fn new(
        text: String,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
        translation: Vec3,
    ) -> (Self, Text2d, TextFont, TextColor, Transform) {
        (
            Self,
            Text2d::new(text),
            TextFont {
                font,
                font_size,
                ..Default::default()
            },
            TextColor(color),
            Transform::from_translation(translation),
        )
    }

    fn from_image(
        image: Handle<Image>,
        translation: Vec3,
        scale: Vec3,
    ) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite::from_image(image),
            Transform {
                translation,
                scale,
                ..Default::default()
            }
        )
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<MyCamera>>,
    score: Res<Score>,
) {
    // debug!("setup");
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = camera_transform.translation.y;
    let font = asset_server.load(PATH_FONT);
    // game over
    let translation = Vec3::new(
        0.0,
        camera_y + TEXT_PADDING * 2.0,
        0.0,
    );
    commands.spawn(Gameover::new(
        GAMEOVER_TEXT.to_string(), 
        font.clone(),
        GAMEOVER_SIZE, 
        TEXT_COLOR, 
        translation,
    ));
    // score
    let translation = Vec3::new(
        0.0,
        camera_y + TEXT_PADDING * 1.0,
        0.0,
    );
    commands.spawn(Gameover::new(
        format!("{}{}", SCORE_TEXT, score.sum()), 
        font.clone(),
        TEXT_SIZE, 
        TEXT_COLOR, 
        translation,
    ));
    // fighter image
    let image = asset_server.load(PATH_IMAGE_FIGHTER);
    let translation = Vec3::new(
        -TEXT_PADDING * 1.0,
        camera_y + TEXT_PADDING * 0.5, 
        0.0,
    );
    commands.spawn(Gameover::from_image(image, translation, FIGHTER_SCALE));
    // fighter score
    let translation = Vec3::new(
        TEXT_PADDING * 0.2,
        camera_y + TEXT_PADDING * 0.5,
        0.0,
    );
    commands.spawn(Gameover::new(
        format!(" x {} ({})", score.fighter, score.sum_fighter()), 
        font.clone(),
        TEXT_SIZE, 
        TEXT_COLOR, 
        translation,
    ));
    // torpedo image
    let image = asset_server.load(PATH_IMAGE_TORPEDO);
    let translation = Vec3::new(
        -TEXT_PADDING * 1.0,
        camera_y - TEXT_PADDING * 0.0,
        0.0,
    );
    commands.spawn(Gameover::from_image(image, translation, TORPEDO_SCALE));
    // torpedo score
    let translation = Vec3::new(
        TEXT_PADDING * 0.2,
        camera_y - TEXT_PADDING * 0.0,
        0.0,
    );
    commands.spawn(Gameover::new(
        format!(" x {} ({})", score.torpedo, score.sum_torpedo()), 
        font.clone(),
        TEXT_SIZE, 
        TEXT_COLOR, 
        translation,
    ));
    // retry
    let translation = Vec3::new(
        0.0,
        camera_y - TEXT_PADDING * 1.0,
        0.0,
    );
    commands.spawn(Gameover::new(
        RETRY_TEXT.to_string(), 
        font.clone(),
        TEXT_SIZE, 
        TEXT_COLOR, 
        translation,
    ));
    // back to title
    let translation = Vec3::new(
        0.0,
        camera_y - TEXT_PADDING * 2.0,
        0.0,
    );
    commands.spawn(Gameover::new(
        BACKTOTITLE_TEXT.to_string(), 
        font.clone(),
        TEXT_SIZE, 
        TEXT_COLOR, 
        translation,
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
    let mut closure = |app_state: AppState| {
        // trace!("AppState Gameover -> {:?}", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => closure(AppState::Ingame),
            KeyCode::KeyB => closure(AppState::Mainmenu),
            _ => {},
        }
    }
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Gameover>>,
) {
    // debug!("all_despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), all_despawn)
        ;
    }
}

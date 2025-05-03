use bevy::prelude::*;

use crate::{
    GAMETITLE,
    PATH_FONT,
    AppState,
};

const GAMETITLE_SIZE: f32 = 20.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CLICKSTART_TEXT: &str = "クリックしてスタート";
const CLICKSTART_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 16.0;

#[derive(Component)]
struct Mainmenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    // game title
    let (x, y, z) = (
        0.0,
        BOARD_SIZE.y / 4.0,
        0.0,
    );
    commands.spawn((
        Text2d::new(GAMETITLE),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: GAMETITLE_SIZE,
            ..Default::default()
        },
        TextColor(GAMETITLE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(x, y, z),
        Mainmenu,
    ));
    // click start
    let (x, y, z) = (
        0.0,
        -BOARD_SIZE.y / 4.0,
        0.0,
    );
   commands.spawn((
        Text2d::new(CLICKSTART_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(CLICKSTART_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(x, y, z),
        Mainmenu,
    ));
    // board
    let (x, y, z) = (
        0.0,
        0.0,
        -10.0,
    );
    commands.spawn((
        Sprite {
            color: BOARD_COLOR,
            custom_size: Some(BOARD_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(x, y, z),
        Mainmenu,
    ));
}

fn update(
    mut next_state: ResMut<NextState<AppState>>,
    mouse_events: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }
    // trace!("AppState Mainmenu -> Ingame");
    next_state.set(AppState::Ingame);
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Mainmenu>>,
) {
    // debug!("all_despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Mainmenu)))
            .add_systems(OnExit(AppState::Mainmenu), all_despawn)
        ;
    }
}

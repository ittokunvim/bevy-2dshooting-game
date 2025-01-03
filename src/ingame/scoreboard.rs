use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Score,
};
use crate::ingame::player::Player;

const SCORE_TEXT: &str = "スコア: ";
const LIFE_TEXT: &str = "ライフ: ";
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_PADDING: f32 = 5.0;

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LifeText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    // score
    let (top, left) = (
        Val::Px(TEXT_PADDING),
        Val::Px(TEXT_PADDING),
    );
    commands.spawn((
        Text::new(SCORE_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top,
            left,
            ..Default::default()
        },
        ScoreboardUi,
    ))
    .with_child((
        TextSpan::default(),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        ScoreText,
    ));
    // life
    let (top, left) = (
        Val::Px(TEXT_PADDING),
        Val::Px(TEXT_PADDING + WINDOW_SIZE.x / 4.0),
    );
    commands.spawn((
        Text::new(LIFE_TEXT),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            top,
            left,
            ..Default::default()
        },
        ScoreboardUi,
    ))
    .with_child((
        TextSpan::default(),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        LifeText,
    ));
}

fn update_score(
    score: Res<Score>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) {
    let Ok(mut span) = query.get_single_mut() else { return };
    // update score
    **span = score.to_string();
}

fn update_life(
    mut life_query: Query<&mut TextSpan, With<LifeText>>,
    player_query: Query<&Player, With<Player>>,
) {
    let Ok(mut span) = life_query.get_single_mut() else { return };
    let Ok(player) = player_query.get_single() else { return };
    // update player hp
    **span = player.hp.to_string();
}

fn reset_score(mut score: ResMut<Score>) {
    // debug!("reset_score");
    **score = 0;
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<ScoreboardUi>>,
) {
    // debug!("all_despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update_score,
                update_life,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
            .add_systems(OnExit(AppState::Gameover), reset_score)
        ;
    }
}

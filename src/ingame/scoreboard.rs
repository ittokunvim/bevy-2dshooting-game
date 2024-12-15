use bevy::prelude::*;

use crate::{AppState, PATH_FONT};
use crate::ingame::Score;

const SCORE_TEXT: &str = "スコア: ";
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct ScoreText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("scoreboard: setup");
    // score
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
            top: TEXT_PADDING,
            left: TEXT_PADDING,
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
        ScoreText,
    ));
}

fn update(
    score: Res<Score>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) {
    // println!("scoreboard: update");
    for mut span in &mut query {
        **span = format!("{}", **score);
    }
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

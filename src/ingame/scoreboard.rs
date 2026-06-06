use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    AppState,
    Score,
};
use crate::ingame::player::{
    PLAYER_HP,
    Player,
};

const PATH_IMAGE: &str = "images/hp-heart.png";

const SCORE_TEXT: &str = "スコア: ";
const LIFE_TEXT: &str = "ライフ: ";
const LIFE_TEXT_WIDTH: f32 = 60.0;

const HEART_SIZE: Vec2 = Vec2::splat(12.8);
const HEART_WRAP: usize = 5;

const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_PADDING: f32 = 5.0;

#[derive(Resource, Deref, DerefMut)]
struct HeartCount(usize);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Heart(usize);

impl ScoreboardUi {
    /// スコアボードのテキストを生成します
    ///
    /// Returns:
    /// * `Self`: ScoreboardUiのインスタンス
    /// * `Text`: 文字
    /// * `TextFont`: フォント
    /// * `TextColor`: 文字色
    /// * `Node`: 位置
    fn from_text(
        text: &str,
        font: Handle<Font>,
        top: Val,
        left: Val,
    ) -> (Self, Text, TextFont, TextColor, Node) {
        (
            Self,
            Text::new(text),
            TextFont {
                font,
                font_size: TEXT_SIZE,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
            Node {
                position_type: PositionType::Absolute,
                top,
                left,
                ..Default::default()
            }
        )
    }

    /// スコアボードのスコアを表示するテキストを生成します
    ///
    /// Returns:
    /// * `Self`: ScoreboardUiのインスタンス
    /// * `TextSpan`: スコア
    /// * `TextFont`: フォント
    /// * `TextColor`: 文字色
    fn from_span(font: Handle<Font>) -> (Self, TextSpan, TextFont, TextColor) {
        (
            Self,
            TextSpan::default(),
            TextFont {
                font,
                font_size: TEXT_SIZE,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
        )
    }

    /// スコアボードにプレイヤーHPを表す画像を生成します
    ///
    /// Returns:
    /// * `Self`: ScoreboardUiのインスタンス
    /// * `TextSpan`: スコア
    /// * `TextFont`: フォント
    /// * `TextColor`: 文字色
    fn from_image(
        image: Handle<Image>,
        top: Val,
        left: Val,
    ) -> (Self, ImageNode, Node) {
        (
            Self,
            ImageNode::new(image),
            Node {
                width: px(HEART_SIZE.x),
                height: px(HEART_SIZE.y),
                top,
                left,
                ..Default::default()
            },
        )
    }
}

/// スコアボードのセットアップを行う関数
/// 構造：
/// * score text
///   * score span
/// * player text
/// * player heart images
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // スコアテキストを生成
    let font = asset_server.load(PATH_FONT);
    let (top, left) = (px(TEXT_PADDING), px(TEXT_PADDING));

    commands.spawn((
        ScoreboardUi::from_text(SCORE_TEXT, font.clone(), top, left),
        children![(
            ScoreboardUi::from_span(font.clone()),
            ScoreText,
        )]
    ));

    // プレイヤーのHPを表示
    let (top, left) = (px(TEXT_PADDING), px(WINDOW_SIZE.x / 2.0 + TEXT_PADDING));

    commands.spawn(ScoreboardUi::from_text(LIFE_TEXT, font.clone(), top, left));

    // プレイヤーのHPを表す画像を表示
    let image = asset_server.load(PATH_IMAGE);

    for i in 1..=PLAYER_HP {
        let row = (i - 1) / HEART_WRAP;
        let col = (i - 1) % HEART_WRAP;

        let top = px(TEXT_PADDING + HEART_SIZE.y * row as f32);
        let left = px(
            WINDOW_SIZE.x / 2.0 + TEXT_PADDING + LIFE_TEXT_WIDTH +
            HEART_SIZE.x * col as f32
        );

        commands.spawn((
            ScoreboardUi::from_image(image.clone(), top, left),
            Heart(i),
        ));
    }
}

/// スコアの更新を検知してテキストに反映させる関数
fn update_score(
    score: Res<Score>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) -> Result {
    info_once!("update_score");

    let mut span = query.single_mut()?;

    **span = score.sum().to_string();

    Ok(())
}

/// プレイヤーのHPの更新を検知してプレイヤーのハートの数を更新する関数
fn update_playerhp(
    mut commands: Commands,
    mut count: ResMut<HeartCount>,
    heart_query: Query<(&Heart, Entity), With<Heart>>,
    player_query: Query<&Player, With<Player>>,
) -> Result {
    info_once!("update_playerhp");

    let player = player_query.single()?;

    // プレイヤーのHPとハートの数が違ったら（値が更新されたら）
    if player.hp != **count {
        for (heart, heart_entity) in &heart_query {
            if player.hp == heart.0 - 1 {
                trace!("count: {}", **count);
                **count -= 1;
                commands.entity(heart_entity).despawn();
            }
        }
    }

    Ok(())
}

/// スコアボードを削除する関数
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<ScoreboardUi>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// スコア、ハートの数をリセットする関数
fn reset(
    mut score: ResMut<Score>,
    mut count: ResMut<HeartCount>,
) {
    info_once!("reset");

    *score = Score::reset();
    **count = PLAYER_HP;
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HeartCount(PLAYER_HP))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update_score,
                update_playerhp,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), despawn)
            .add_systems(OnExit(AppState::Gameover), reset)
        ;
    }
}

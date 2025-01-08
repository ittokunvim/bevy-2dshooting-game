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
const HEART_SCALE: Vec3 = Vec3::splat(0.2);
const HEART_MARGIN: f32 = 26.0;
const HEART_WRAP: usize = 5;
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PADDING: f32 = 5.0;

#[derive(Resource, Deref, DerefMut)]
struct HeartCount(usize);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Heart(usize);

impl ScoreboardUi {
    fn new_text(
        text: String,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
        top: Val,
        left: Val,
    ) -> (Self, Text, TextFont, TextColor, Node) {
        (
            Self,
            Text::new(text),
            Self::textfont(font, font_size),
            TextColor(color),
            Self::node(top, left),
        )
    }

    fn new_span(
        font: Handle<Font>,
        font_size: f32,
        color: Color,
    ) -> (Self, TextFont, TextColor) {
        (
            Self,
            Self::textfont(font, font_size),
            TextColor(color),
        )
    }

    fn new_image(
        image: Handle<Image>,
        scale: Vec3,
        top: Val,
        left: Val,
    ) -> (Self, ImageNode, Transform, Node) {
        (
            Self,
            ImageNode::new(image),
            Transform::from_scale(scale),
            Self::node(top, left),
        )
    }

    fn textfont(
        font: Handle<Font>,
        font_size: f32,
    ) -> TextFont {
        TextFont {
            font,
            font_size,
            ..Default::default()
        }
    }

    fn node(
        top: Val,
        left: Val,
    ) -> Node {
        Node {
            position_type: PositionType::Absolute,
            top,
            left,
            ..Default::default()
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let font = asset_server.load(PATH_FONT);
    // score
    let (top, left) = (
        Val::Px(PADDING),
        Val::Px(PADDING),
    );
    commands.spawn(ScoreboardUi::new_text(
        SCORE_TEXT.to_string(),
        font.clone(),
        TEXT_SIZE,
        TEXT_COLOR,
        top,
        left,
    ))
    .with_child(ScoreboardUi::new_span(
        font.clone(),
        TEXT_SIZE,
        TEXT_COLOR,
    ));
    // player hp
    let (top, left) = (
        Val::Px(PADDING),
        Val::Px(WINDOW_SIZE.x / 2.0 + PADDING),
    );
    commands.spawn(ScoreboardUi::new_text(
        LIFE_TEXT.to_string(), 
        font.clone(), 
        TEXT_SIZE, 
        TEXT_COLOR, 
        top, 
        left,
    ));
    // player hp heart
    let image = asset_server.load(PATH_IMAGE);
    for i in 1..PLAYER_HP + 1 {
        let (mut top, mut left) = (
            Val::Px(PADDING - HEART_MARGIN),
            Val::Px(
                WINDOW_SIZE.x / 2.0 + PADDING + LIFE_TEXT_WIDTH - HEART_MARGIN +
                HEART_SIZE.x * i as f32),
        );
        if i > HEART_WRAP {
            top = Val::Px(PADDING - HEART_MARGIN + HEART_SIZE.y);
            left = Val::Px(
                WINDOW_SIZE.x / 2.0 + PADDING + LIFE_TEXT_WIDTH - HEART_MARGIN +
                HEART_SIZE.x * (i - HEART_WRAP) as f32);
        }
        commands.spawn((
            ScoreboardUi::new_image(
                image.clone(),
                HEART_SCALE,
                top,
                left,
            ),
            Heart(i),
        ));
    }
}

fn update_score(
    score: Res<Score>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) {
    let Ok(mut span) = query.get_single_mut() else { return };
    // update score
    **span = score.sum().to_string();
}

fn update_playerhp(
    mut commands: Commands,
    mut count: ResMut<HeartCount>,
    heart_query: Query<(&Heart, Entity), With<Heart>>,
    player_query: Query<&Player, With<Player>>,
) {
    let Ok(player) = player_query.get_single() else { return };

    if player.hp == **count { return }

    for (heart, heart_entity) in &heart_query {
        if player.hp != heart.0 - 1 { continue }
        **count -= 1;
        // trace!("count: {}", **count);
        commands.entity(heart_entity).despawn();
    }
}

fn reset_score(mut score: ResMut<Score>) {
    // debug!("reset_score");
    *score = Score::reset();
}

fn reset_count(mut count: ResMut<HeartCount>) {
    // debug!("reset_count");
    **count = PLAYER_HP;
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
            .insert_resource(HeartCount(PLAYER_HP))
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update_score,
                update_playerhp,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
            .add_systems(OnExit(AppState::Gameover), (
                reset_score,
                reset_count,
            ))
        ;
    }
}

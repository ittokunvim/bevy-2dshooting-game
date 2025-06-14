use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
    MyCamera,
};

const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_IMAGE: &str = "ittoku-2dshooting-game/background.png";
const IMAGE_SIZE: UVec2 = UVec2::new(640, 480);
const COLUMN: u32 = 9;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(1.0);
const FPS: f32 = 0.1;
const MAX_COUNT: usize = 3;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Mybackground;

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let images = asset_server.load(PATH_IMAGE);

    for i in 0..MAX_COUNT {
        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 8, };
        let translation = Vec3::new(
            0.0,
            WINDOW_SIZE.y * i as f32,
            -99.0,
        );
        // background
        commands.spawn((
            Sprite::from_atlas_image(
                images.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform {
                translation,
                scale: SCALE,
                ..Default::default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(FPS, TimerMode::Repeating)),
            Mybackground,
        ));
    }
}

fn animation(
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    time: Res<Time>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last
                    { indices.first } else { atlas.index + 1 }
            }
        }
    }
}

fn check_offscreen(
    mut bg_query: Query<&mut Transform, (With<Mybackground>, Without<MyCamera>)>,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Mybackground>)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = (camera_transform.translation.y / 10.0).round() * 10.0;

    for mut bg_transform in &mut bg_query {
        let bg_y = bg_transform.translation.y;

        if bg_y <= camera_y - WINDOW_SIZE.y + 5.0 {
            // debug!("check_offscreen");
            bg_transform.translation.y = camera_y + WINDOW_SIZE.y * 2.0;
            // trace!("background y: {}", bg_transform.translation.y);
        }
    }
}

fn reset_position(
    mut query: Query<&mut Transform, With<Mybackground>>
) {
    // debug!("reset_position");
    let mut i = 0;

    for mut transform in &mut query {
        transform.translation.y = WINDOW_SIZE.y * i as f32;
        i += 1;
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                animation,
                check_offscreen,
            ))
            .add_systems(OnExit(AppState::Gameover), reset_position)
        ;
    }
}

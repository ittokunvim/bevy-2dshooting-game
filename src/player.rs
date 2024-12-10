use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    GRID_SIZE,
    PATH_IMAGE_PLAYER,
};

const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(2.0);
const TRANSLATION: Vec3 = Vec3::new(0.0, GRID_SIZE * -12.0, 99.0);
const SPEED: f32 = 200.0;
const SIZE: f32 = 64.0;

#[derive(Component)]
struct Player {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    println!("player: setup");
    let texture = asset_server.load(PATH_IMAGE_PLAYER);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Player { first: 0, last: 3, };

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
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
        ;
    }
}

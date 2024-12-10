use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    PATH_IMAGE_BULLET,
    Player,
};

const IMAGE_SIZE: UVec2 = UVec2::splat(32);
const COLUMN: u32 = 4;
const ROW: u32 = 1;

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Component)]
struct Bullet {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_BULLET);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    bullet_image: Res<BulletImage>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) { return }

    println!("bullet: shoot");
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Bullet { first: 0, last: 3, };
    let player_transform = player_query.single();
    let translation = Vec3::new(
        player_transform.translation.x, 
        player_transform.translation.y + GRID_SIZE * 2.0, 
        99.0,
    );

    commands.spawn((
        Sprite::from_atlas_image(
            bullet_image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_translation(translation),
        animation_indices,
    ));
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, shoot)
        ;
    }
}

use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_IMAGE_BACKGROUND: &str = "bevy-2dshooting-game/background.png";
const IMAGE_SIZE: UVec2 = UVec2::new(640, 480);
const COLUMN: u32 = 9;
const ROW: u32 = 1;
const TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -99.0);
const SCALE: Vec3 = Vec3::splat(1.0);
const FPS: f32 = 0.1;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    println!("background: setup");
    let texture = asset_server.load(PATH_IMAGE_BACKGROUND);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 8, };

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
        AnimationTimer(Timer::from_seconds(FPS, TimerMode::Repeating)),
    ));
}

fn update(
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

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, update)
        ;
    }
}

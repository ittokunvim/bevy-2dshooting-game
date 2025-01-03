use bevy::prelude::*;

use crate::AppState;
use crate::ingame::fighter::ShipDespawnEvent;
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "bevy-2dshooting-game/enemy-despawn.png";
const IMAGE_SIZE: UVec2 = UVec2::splat(64);
const COLUMN: u32 = 9;
const ROW: u32 = 1;
const FPS: f32 = 0.1;

#[derive(Resource, Deref)]
struct DespawnImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(DespawnImage(handle));
}

fn spawn(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut events: EventReader<ShipDespawnEvent>,
    despawn_image: Res<DespawnImage>,
) {
    for event in events.read() {
        let vec2 = event.0;
        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let translation = Vec3::new(
            vec2.x, 
            vec2.y,
            99.0,
        );

        let animation_config = AnimationConfig::new(AnimationName::Despawn, 0, 8, FPS);
        // debug!("spawn");
        commands.spawn((
            Sprite::from_atlas_image(
                despawn_image.clone(), 
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_config.first_sprite_index,
                },
            ),
            Transform::from_translation(translation),
            animation_config,
        ));
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, spawn.run_if(in_state(AppState::Ingame)))
        ;
    }
}

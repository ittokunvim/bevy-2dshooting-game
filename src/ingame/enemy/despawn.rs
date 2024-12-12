use bevy::prelude::*;

use crate::AppState;
use crate::ingame::EnemyShip;
use crate::ingame::enemy::EnemyDespawnEvent;

const IMAGE_SIZE: UVec2 = UVec2::splat(64);
const COLUMN: u32 = 9;
const ROW: u32 = 1;
const FPS: f32 = 0.1;

const PATH_IMAGE_ENEMY_DESPAWN: &str = "bevy-2dshooting-game/enemy-despawn.png";

#[derive(Resource, Deref)]
struct DespawnImage(Handle<Image>);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("enemy.despawn: setup");
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE_ENEMY_DESPAWN);
    commands.insert_resource(DespawnImage(handle));
}

fn spawn_despawn(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut events: EventReader<EnemyDespawnEvent>,
    despawn_image: Res<DespawnImage>,
    enemy_query: Query<&Transform, With<EnemyShip>>,
) {
    if events.is_empty() { return }
    events.clear();

    for enemy_transform in &enemy_query {
        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 8, };
        let translation = Vec3::new(
            enemy_transform.translation.x, 
            enemy_transform.translation.y,
            99.0,
        );
        // println!("enemy.despawn: spawn");
        commands.spawn((
            Sprite::from_atlas_image(
                despawn_image.clone(), 
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform::from_translation(translation),
            animation_indices,
            AnimationTimer(Timer::from_seconds(FPS, TimerMode::Repeating)),
        ));
    }
}

fn animation(
    mut commands: Commands,
    mut query: Query<(Entity, &AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == indices.last {
                    commands.entity(entity).despawn();
                }
                atlas.index += 1;
            }
        }
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                spawn_despawn,
                animation,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}

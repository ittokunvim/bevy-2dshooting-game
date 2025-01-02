use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GRID_SIZE;
use crate::ingame::fighter::Fighter;
use crate::ingame::utils::prelude::*;

const PATH_IMAGE: &str = "bevy-2dshooting-game/fighter-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(4, 16);
const COLUMN: u32 = 4;
const ROW: u32 = 1;
const DEGREES: f32 = 180.0;
const SCALE: Vec3 = Vec3::splat(2.0);
const DIRECTION: Vec2 = Vec2::new(0.0, -1.0);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(8.0, 32.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load(PATH_IMAGE);
    commands.insert_resource(BulletImage(handle));
}

fn shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut fighter_query: Query<(&mut Fighter, &Transform), With<Fighter>>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    for (mut fighter, fighter_transform) in &mut fighter_query {
        if !fighter.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let translation = Vec3::new(
            fighter_transform.translation.x, 
            fighter_transform.translation.y - GRID_SIZE * 2.0, 
            99.0,
        );

        let animation_config = AnimationConfig::new(AnimationName::Bullet, 0, 3, FPS);
        let velocity = Velocity(DIRECTION * SPEED);
        let bullet = Bullet::new(
            Shooter::Fighter,
            SIZE, 
            bullet_image.clone(), 
            texture_atlas_layout.clone(), 
            animation_config.first_sprite_index, 
            translation, 
            DEGREES, 
            SCALE,
        );
        // bullet
        commands.spawn((bullet, animation_config, velocity));
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, shoot.run_if(in_state(AppState::Ingame)))
        ;
    }
}

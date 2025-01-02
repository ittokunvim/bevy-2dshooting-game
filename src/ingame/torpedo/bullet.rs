use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
};

use crate::{
    WINDOW_SIZE,
    AppState,
    MyCamera,
};
use crate::ingame::GRID_SIZE;
use crate::ingame::player::{
    PlayerDamageEvent,
    Player,
};
use crate::ingame::torpedo::Torpedo;
use crate::ingame::utils::animation_config::{
    AnimationConfig,
    AnimationName,
};
use crate::ingame::utils::velocity::Velocity;

const PATH_IMAGE: &str = "bevy-2dshooting-game/torpedo-bullet.png";
const IMAGE_SIZE: UVec2 = UVec2::new(11, 32);
const COLUMN: u32 = 3;
const ROW: u32 = 1;
const SCALE: Vec3 = Vec3::splat(1.5);
const SPEED: f32 = 256.0;
const FPS: f32 = 0.1;
const SIZE: Vec2 = Vec2::new(16.5, 48.0);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

#[derive(Component)]
#[require(Sprite, Transform)]
struct Bullet {
    size: Vec2,
}

impl Bullet {
    pub fn new(
        size: Vec2,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        first_index: usize,
        translation: Vec3,
        degrees: f32,
        scale: Vec3,
    ) -> (Self, Sprite, Transform) {
        (
            Self { size, },
            Self::sprite(image, layout, first_index),
            Self::transform(translation, degrees, scale)
        )
    }

    fn sprite(
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
    ) -> Sprite {
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout,
                index,
            }
        )
    }

    fn transform(
        translation: Vec3,
        degrees: f32,
        scale: Vec3,
    ) -> Transform {
        Transform {
            translation,
            rotation: Quat::from_rotation_z(degrees.to_radians()),
            scale,
        }
    }
}

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
    mut torpedo_query: Query<(&mut Torpedo, &Transform), (With<Torpedo>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Torpedo>)>,
    bullet_image: Res<BulletImage>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };

    for (mut torpedo, torpedo_transform) in &mut torpedo_query {
        if !torpedo.shoot_timer.tick(time.delta()).just_finished() { continue }

        let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, COLUMN, ROW, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let torpedo_xy = torpedo_transform.translation.xy();
        let translation = Vec3::new(
            torpedo_xy.x, 
            torpedo_xy.y - GRID_SIZE * 2.0, 
            99.0,
        );
        let player_xy = player_transform.translation.xy();
        let delta_xy = (player_xy - translation.xy()).normalize();
        let degrees = delta_xy.y.atan2(delta_xy.x).to_degrees() - 90.0;

        let animation_config = AnimationConfig::new(AnimationName::Bullet, 0, 2, FPS);
        let velocity = Velocity(delta_xy * SPEED);
        let bullet = Bullet::new(
            SIZE,
            bullet_image.clone(),
            texture_atlas_layout.clone(),
            animation_config.first_sprite_index,
            translation,
            degrees,
            SCALE,
        );
        // bullet
        commands.spawn((bullet, animation_config, velocity));
    }
}

fn check_for_hit(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(Entity, &Bullet, &Transform), (With<Bullet>, Without<Player>)>,
    player_query: Query<(&Transform, &Player), (With<Player>, Without<Bullet>)>,
) {
    let Ok((player_transform, player)) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.xy();

    for (bullet_entity, bullet, bullet_transform) in &bullet_query {
        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
            .intersects(&Aabb2d::new(player_pos, player.size / 2.0));

        if collision {
            // damage player
            events.send_default();
            // despawn bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_offscreen(
    mut commands: Commands,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Bullet>)>,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<MyCamera>)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_y = camera_transform.translation.y;

    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_y = bullet_transform.translation.y;
        // check off screen
        if bullet_y <= camera_y - WINDOW_SIZE.y / 2.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bullet>>,
) {
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                shoot,
                check_for_hit,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
        ;
    }
}

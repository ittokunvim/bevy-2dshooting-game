use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, IntersectsVolume},
    window::PrimaryWindow,
};

use crate::{
    AppState,
    MyCamera,
};
use crate::ingame::player::{
    PlayerDamageEvent,
    Player,
};
use crate::ingame::fighter::{
    FighterDamageEvent,
    Fighter,
};
use crate::ingame::torpedo::{
    TorpedoDamageEvent,
    Torpedo,
};

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Bullet {
    shooter: Shooter,
    pub size: Vec2,
}

#[derive(PartialEq)]
pub enum Shooter {
    Player,
    Enemy,
}

impl Bullet {
    pub fn new(
        shooter: Shooter,
        size: Vec2,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        first_index: usize,
        translation: Vec3,
        degrees: f32,
        scale: Vec3,
    ) -> (Self, Sprite, Transform) {
        (
            Self { shooter, size, },
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

fn check_for_hit_player(
    mut commands: Commands,
    mut events: EventWriter<PlayerDamageEvent>,
    bullet_query: Query<(&Bullet, Entity, &Transform), (With<Bullet>, Without<Player>)>,
    player_query: Query<(&Player, &Transform), (With<Player>, Without<Bullet>)>,
) {
    let Ok((player, player_transform)) = player_query.get_single() else { return };
    let player_pos = player_transform.translation.xy();

    for (bullet, bullet_entity, bullet_transform) in &bullet_query {
        if bullet.shooter == Shooter::Player { continue }

        let bullet_pos = bullet_transform.translation.xy();

        let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
            .intersects(&Aabb2d::new(player_pos, player.size / 2.0));

        if collision {
            // debug!("check_for_hit_player");
            events.send_default();
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_hit_fighter(
    mut commands: Commands,
    mut events: EventWriter<FighterDamageEvent>,
    mut player_query: Query<&mut Player, With<Player>>,
    bullet_query: Query<(&Bullet, Entity, &Transform), (With<Bullet>, Without<Fighter>)>,
    fighter_query: Query<(&Fighter, Entity, &Transform), (With<Fighter>, Without<Bullet>)>,
) {
    for (bullet, bullet_entity, bullet_transform) in &bullet_query {
        if bullet.shooter != Shooter::Player { continue }

        let bullet_pos = bullet_transform.translation.xy();
        let mut is_hit_bullet = false;

        for (fighter, fighter_entity, fighter_transform) in &fighter_query {
            let fighter_pos = fighter_transform.translation.xy();

            let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
                .intersects(&Aabb2d::new(fighter_pos, fighter.size / 2.0));

            if collision {
                let Ok(mut player) = player_query.get_single_mut() else { return };
                // debug!("check_for_hit_fighter");
                is_hit_bullet = true;
                player.bullets += 1;
                events.send(FighterDamageEvent(fighter_entity));
            }
        }
        if is_hit_bullet {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_hit_torpedo(
    mut commands: Commands,
    mut events: EventWriter<TorpedoDamageEvent>,
    mut player_query: Query<&mut Player, With<Player>>,
    bullet_query: Query<(&Bullet, Entity, &Transform), (With<Bullet>, Without<Torpedo>)>,
    torpedo_query: Query<(&Torpedo, Entity, &Transform), (With<Torpedo>, Without<Bullet>)>,
) {
    for (bullet, bullet_entity, bullet_transform) in &bullet_query {
        if bullet.shooter != Shooter::Player { continue }

        let bullet_pos = bullet_transform.translation.xy();
        let mut is_hit_bullet = false;

        for (torpedo, torpedo_entity, torpedo_transform) in &torpedo_query {
            let torpedo_pos = torpedo_transform.translation.xy();
            let collision = Aabb2d::new(bullet_pos, bullet.size / 2.0)
                .intersects(&Aabb2d::new(torpedo_pos, torpedo.size / 2.0));

            if collision {
                let Ok(mut player) = player_query.get_single_mut() else { return };
                // debug!("check_for_hit_torpedo");
                is_hit_bullet = true;
                player.bullets += 1;
                events.send(TorpedoDamageEvent(torpedo_entity));
            }
        }
        if is_hit_bullet {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn check_for_offscreen(
    mut commands: Commands,
    mut player_query: Query<&mut Player, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, (With<MyCamera>, Without<Bullet>)>,
    bullet_query: Query<(&Bullet, Entity, &Transform), (With<Bullet>, Without<MyCamera>)>,
) {
    let Ok(window) = window_query.get_single() else { return };
    let (window_width, window_height) = (window.width(), window.height());
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let camera_pos = camera_transform.translation.xy();

    for (bullet, bullet_entity, bullet_transform) in &bullet_query {
        let bullet_x = bullet_transform.translation.x;
        let bullet_y = bullet_transform.translation.y;
        let (bullet_width, bullet_height) = (bullet.size.x, bullet.size.y);
        let left_bound   = camera_pos.x - window_width  / 2.0 - bullet_width  / 2.0;
        let right_bound  = camera_pos.x + window_width  / 2.0 + bullet_width  / 2.0;
        let bottom_bound = camera_pos.y - window_height / 2.0 - bullet_height / 2.0;
        let top_bound    = camera_pos.y + window_height / 2.0 + bullet_height / 2.0;

        if bullet_x <= left_bound || bullet_x >= right_bound
        || bullet_y <= bottom_bound || bullet_y >= top_bound {
            if bullet.shooter == Shooter::Player {
                let Ok(mut player) = player_query.get_single_mut() else { return };
                // trace!("player.bullets: {}", player.bullets);
                player.bullets += 1;
            }
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn all_despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bullet>>,
) {
    // debug!("all_despawn");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                check_for_hit_player,
                check_for_hit_fighter,
                check_for_hit_torpedo,
                check_for_offscreen,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), all_despawn)
        ;
    }
}

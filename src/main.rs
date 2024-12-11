use bevy::prelude::*;

mod enemy;
mod player;

const GAMETITLE: &str = "2Dシューティングゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const GRID_SIZE: f32 = 16.0;
const PATH_IMAGE_PLAYER_SHIP: &str = "bevy-2dshooting-game/player-ship.png";
const PATH_IMAGE_PLAYER_BULLET: &str = "bevy-2dshooting-game/player-bullet.png";
const PATH_IMAGE_ENEMY_SHIP: &str = "bevy-2dshooting-game/enemy-ship.png";
const PATH_IMAGE_ENEMY_BULLET: &str = "bevy-2dshooting-game/enemy-bullet.png";
const PATH_SOUND_SHOOT: &str = "bevy-2dshooting-game/shoot.ogg";

const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
struct PlayerShip {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct PlayerBullet {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct EnemyShip;

#[derive(Component)]
struct EnemyBullet {
    first: usize,
    last: usize,
}

#[derive(Event, Default)]
struct EnemyBulletHitEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    println!("main: setup");
    commands.spawn(Camera2d::default());
}

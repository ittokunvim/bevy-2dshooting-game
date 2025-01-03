use bevy::{
    prelude::*,
    log::LogPlugin,
};

mod background;
mod mainmenu;
mod ingame;
mod gameover;

const GAMETITLE: &str = "2Dシューティングゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Mainmenu,
    Ingame,
    Gameover,
}

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct MyCamera;

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
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_2dshooting_game=trace".into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
        )
        .insert_state(AppState::Mainmenu)
        .insert_resource(Score(0))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // debug!("setup camera");
    commands.spawn((Camera2d::default(), MyCamera));
}

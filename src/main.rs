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

const SCORE_FIGHTER: usize = 10;
const SCORE_TORPEDO: usize = 50;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Mainmenu,
    Ingame,
    Gameover,
}

#[derive(Resource)]
struct Score {
    fighter: usize,
    torpedo: usize,
}

#[derive(Component)]
struct MyCamera;

impl Score {
    fn new() -> Self {
        Self { fighter: 0, torpedo: 0, }
    }

    fn sum_fighter(&self) -> usize { SCORE_FIGHTER * self.fighter }

    fn sum_torpedo(&self) -> usize { SCORE_TORPEDO * self.torpedo }

    fn sum(&self) -> usize { self.sum_fighter() + self.sum_torpedo() }

    fn reset() -> Self {
        Self::new()
    }
}

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
        .insert_resource(Score::new())
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

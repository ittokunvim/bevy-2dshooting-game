use bevy::{
  asset::AssetMetaCheck,
  log::LogPlugin,
  prelude::*,
  window::WindowResolution
};

mod background;
mod mainmenu;
mod ingame;
mod gameover;

const GAMETITLE: &str = "いっとく2Dシューティングゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";

const SCORE_FIGHTER: usize = 10;
const SCORE_TORPEDO: usize = 50;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Resource)]
enum AppState {
    #[default]
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

    fn reset() -> Self { Self::new() }
}

fn main() {
    let window_size = WINDOW_SIZE.as_uvec2();

    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(window_size.x, window_size.y),
                    title: GAMETITLE.to_string(),
                    canvas: Some("#bevy".into()),
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
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
        )
        .init_state::<AppState>()
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

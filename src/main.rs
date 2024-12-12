use bevy::prelude::*;

mod mainmenu;
mod ingame;

const GAMETITLE: &str = "2Dシューティングゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Mainmenu,
    Ingame,
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
        )
        .insert_state(AppState::Mainmenu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    println!("main: setup");
    commands.spawn(Camera2d::default());
}

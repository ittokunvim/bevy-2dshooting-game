use bevy::prelude::*;

use crate::AppState;
use crate::ingame::player::ShootEvent;

const PATH_SOUND_SHOOT: &str = "sounds/battle-shooting-1.ogg";

#[derive(Resource, Deref)]
struct ShootSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let handle = asset_server.load(PATH_SOUND_SHOOT);
    commands.insert_resource(ShootSound(handle));
}

fn play_shoot_sound(
    _shot: On<ShootEvent>,
    mut commands: Commands,
    sound: Res<ShootSound>,
) {
    // debug!("play_shoot_sound");
    commands.spawn((
        AudioPlayer(sound.clone()),
        PlaybackSettings::DESPAWN,
    ));
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_observer(play_shoot_sound)
        ;
    }
}

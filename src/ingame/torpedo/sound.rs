use bevy::prelude::*;

use crate::AppState;
use crate::ingame::torpedo::TorpedoDamageEvent;

const PATH_SOUND: &str = "sounds/battle-shooting-hit.ogg";

#[derive(Resource, Deref)]
struct DespawnSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // debug!("setup");
    let handle = asset_server.load(PATH_SOUND);
    commands.insert_resource(DespawnSound(handle));
}


fn play_damage_sound(
    _damaged: On<TorpedoDamageEvent>,
    mut commands: Commands,
    sound: Res<DespawnSound>,
) {
    // debug!("play_damage_sound");
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
            .add_observer(play_damage_sound)
        ;
    }
}

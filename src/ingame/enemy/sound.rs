use bevy::prelude::*;

use crate::AppState;
use crate::ingame::enemy::ShipDespawnEvent;

const PATH_SOUND_DESPAWN: &str = "sounds/battle-blow-3.ogg";

#[derive(Resource, Deref)]
struct DespawnSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("enemy.sound: setup");
    let handle = asset_server.load(PATH_SOUND_DESPAWN);
    commands.insert_resource(DespawnSound(handle));
}

fn play_shoot_sound(
    mut events: EventReader<ShipDespawnEvent>,
    mut commands: Commands,
    sound: Res<DespawnSound>,
) {
    // println!("enemy.sound: play_shoot_sound");
    if events.is_empty() { return }
    events.clear();
    // despawn sound
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
            .add_systems(Update, play_shoot_sound.run_if(in_state(AppState::Ingame)))
        ;
    }
}

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct ShotgunSound(pub Handle<AudioSource>);

pub struct EnemyShotSound(pub Handle<AudioSource>);

pub struct AudioLoadPlugin;

impl Plugin for AudioLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_audio);
    }
}

fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShotgunSound(asset_server.load("audio/shot.wav")));
    commands.insert_resource(EnemyShotSound(asset_server.load("audio/enemy_shot.wav")));
}

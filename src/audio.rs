use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Deref, DerefMut)]
pub struct ShotgunSound(pub Handle<AudioSource>);

#[derive(Deref, DerefMut)]
pub struct EnemyShotSound(pub Handle<AudioSource>);

#[derive(Deref, DerefMut)]
pub struct FootstepSounds(pub Vec<Handle<AudioSource>>);

pub struct AudioLoadPlugin;

impl Plugin for AudioLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_audio);
    }
}

fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShotgunSound(asset_server.load("audio/shot.wav")));
    commands.insert_resource(EnemyShotSound(asset_server.load("audio/enemy_shot.wav")));

    commands.insert_resource(FootstepSounds(vec![
        asset_server.load("audio/footstep_indoor_1.wav"),
        asset_server.load("audio/footstep_indoor_2.wav"),
        asset_server.load("audio/footstep_indoor_3.wav"),
        asset_server.load("audio/footstep_indoor_4.wav"),
    ]));
}

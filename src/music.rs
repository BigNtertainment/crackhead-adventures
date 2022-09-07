use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource, AudioControl};

use crate::GameState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_music)
		.add_system_set(SystemSet::on_enter(GameState::Game).with_system(play_music))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(stop_music));
    }
}

#[derive(Deref, DerefMut)]
struct Music(Handle<AudioSource>);

fn load_music(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(Music(asset_server.load("audio/song.wav")));
}

fn play_music(audio: Res<Audio>, music: Res<Music>) {
	// audio.play(music.clone()).with_volume(0.0).loop_from(9.54);
}

fn stop_music(audio: Res<Audio>) {
	audio.stop();
}
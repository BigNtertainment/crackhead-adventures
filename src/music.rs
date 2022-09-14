use bevy::prelude::*;
use bevy_kira_audio::{AudioSource, Audio};

use crate::{GameState, audio_player::{AudioPlayer, MUSIC_VOLUME}, settings::Settings};

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
	commands.insert_resource(Music(asset_server.load("./audio/song.wav")));
}

fn play_music(audio: Res<Audio>, music: Res<Music>, settings: Res<Settings>) {
	AudioPlayer::play_music(audio.into_inner(), music.clone(), MUSIC_VOLUME, &settings, 9.54);
}

fn stop_music(audio: Res<Audio>) {
	AudioPlayer::stop(audio.into_inner());
}

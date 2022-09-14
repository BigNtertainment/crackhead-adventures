use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, MainTrack, AudioSource};

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(register_audio_player);
	}
}

pub struct AudioPlayer {
	audio: Audio,
}

impl AudioPlayer {
	pub fn play_sfx(sound: Handle<AudioSource>) {

	}
}

fn register_audio_player(mut commands: Commands) {
	let audio = AudioChannel::<MainTrack>::default();

	commands.insert_resource(AudioPlayer { audio });
}
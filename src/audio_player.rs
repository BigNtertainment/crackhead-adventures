use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource, AudioControl};

use crate::settings::Settings;

pub const MUSIC_VOLUME: f64 = 0.025;
pub const PLAYER_SHOT_VOLUME: f64 = 0.05;
pub const PLAYER_FOOTSTEP_VOLUME: f64 = 1.0;
pub const PLAYER_SNORTING_VOLUME: f64 = 0.1;
pub const PLAYER_CRAFTING_VOLUME: f64 = 0.1;
pub const ENEMY_SHOT_VOLUME: f64 = 0.1;
pub const ENEMY_DEATH_SCREAM_VOLUME: f64 = 0.3;

pub struct AudioPlayer;

impl AudioPlayer {
	pub fn play_sfx(audio: &Audio, sound: Handle<AudioSource>, default_volume: f64, settings: &Settings) {
		audio.play(sound).with_volume(settings.sfx_volume * default_volume);
	}

	pub fn play_music(audio: &Audio, sound: Handle<AudioSource>, default_volume: f64, settings: &Settings, loop_start: f64) {
		audio.play(sound).with_volume(settings.music_volume * default_volume).loop_from(loop_start);
	}

	pub fn stop(audio: &Audio) {
		audio.stop();
	}
}
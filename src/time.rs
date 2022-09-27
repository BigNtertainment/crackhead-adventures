use std::time::Duration;

use bevy::prelude::*;

pub struct TimeCounter {
	pub timescale: f32,
	delta: f32,
	seconds_since_startup: f32,
}

impl TimeCounter {
	pub fn new() -> Self {
		Self {
			timescale: 1.0,
			delta: 0.0,
			seconds_since_startup: 0.0,
		}
	}

	pub fn step(&mut self, delta: f32) {
		self.delta = delta;
		self.seconds_since_startup += delta;
	}

	pub fn delta(&self) -> Duration {
		Duration::from_millis((self.delta_seconds() * 1000.0) as u64)
	}

	pub fn delta_seconds(&self) -> f32 {
		self.timescale * self.delta
	}

	pub fn seconds_since_startup(&self) -> f32 {
		self.seconds_since_startup
	}
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(register_time)
			.add_system_set_to_stage(
				CoreStage::PreUpdate,
				SystemSet::new().with_system(update_time),
			);
	}
}

fn register_time(mut commands: Commands) {
	commands.insert_resource(TimeCounter::new());
}

fn update_time(mut time: ResMut<TimeCounter>, game_time: Res<Time>) {
	time.step(game_time.delta_seconds());
}

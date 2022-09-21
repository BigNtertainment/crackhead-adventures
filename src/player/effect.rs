use bevy::prelude::*;

use crate::{
	time::TimeCounter,
	unit::{Effect, Health, Movement, Shooting},
};

#[derive(Component)]
pub struct EffectData {
	pub effect: Option<Box<dyn Effect + Send + Sync>>,
	pub duration: Timer,
}

impl EffectData {
	pub fn apply(
		&mut self,
		effect: Option<Box<dyn Effect + Send + Sync>>,
		movement: &mut Movement,
		health: &mut Health,
		shooting: &mut Shooting,
		time: &mut TimeCounter,
		duration: f32,
	) {
		self.effect = effect;

		if let Some(effect) = &mut self.effect {
			effect.apply(movement, health, shooting, time);
		}

		self.duration = Timer::from_seconds(duration, false);
	}

	pub fn finish(
		&mut self,
		movement: &mut Movement,
		health: &mut Health,
		shooting: &mut Shooting,
		time: &mut TimeCounter,
	) {
		if let Some(effect) = &self.effect {
			effect.finish(movement, health, shooting, time);
		}

		self.effect = None;
	}
}

pub struct SmallPowerup;

impl Effect for SmallPowerup {
	fn apply(&self, movement: &mut Movement, health: &mut Health, _: &mut Shooting, _: &mut TimeCounter) {
		movement.speed *= 2.0;
		health.heal(35.0);
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health, _: &mut Shooting, _: &mut TimeCounter) {
		movement.speed /= 2.0;
	}
}

pub struct BigPowerup;

impl Effect for BigPowerup {
	fn apply(&self, movement: &mut Movement, _: &mut Health, shooting: &mut Shooting, time: &mut TimeCounter) {
		movement.speed *= 3.0;
		shooting.cooldown.set_duration(shooting.cooldown.duration() / 5);
		time.timescale /= 3.0;
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health, shooting: &mut Shooting, time: &mut TimeCounter) {
		movement.speed /= 3.0;
		shooting.cooldown.set_duration(shooting.cooldown.duration() * 5);
		time.timescale *= 3.0;
	}
}

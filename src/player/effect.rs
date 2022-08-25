use bevy::prelude::*;

use crate::unit::{Effect, Health, Movement};

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
		duration: f32,
	) {
		self.effect = effect;

		if let Some(effect) = &mut self.effect {
			effect.apply(movement, health);
		}

		self.duration = Timer::from_seconds(duration, false);
	}

	pub fn finish(&mut self, movement: &mut Movement, health: &mut Health) {
		if let Some(effect) = &self.effect {
			effect.finish(movement, health);
		}

		self.effect = None;
	}
}

pub struct SmallPowerup;

impl Effect for SmallPowerup {
	fn apply(&self, movement: &mut Movement, health: &mut Health) {
		movement.speed *= 2.0;
		health.heal(35.0);
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health) {
		movement.speed /= 2.0;
	}
}

pub struct BigPowerup;

impl Effect for BigPowerup {
	fn apply(&self, movement: &mut Movement, health: &mut Health) {
		movement.speed *= 5.0;
		if health.get_health() > 10.0 {
			health.set_health(10.0);
		}
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health) {
		movement.speed /= 5.0;
	}
}

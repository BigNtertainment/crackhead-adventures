use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Movement {
	pub speed: f32,
}

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Health {
	health: f32,
	max_health: f32,
}

impl Health {
	pub fn new(max_health: f32) -> Self {
		Self { health: max_health, max_health }
	}

	/// # Returns
	/// True if the health reached zero.
	#[warn(unused_must_use)]
	#[must_use]
	pub fn take_damage(&mut self, amount: f32) -> bool {
		self.health -= amount;

		self.health <= 0.0
	}
	pub fn heal(&mut self, amount: f32) {
		self.health += amount;

		if self.health > self.max_health {
			self.health = self.max_health;
		}
	}

	pub fn get_health(&self) -> f32 {
		self.health
	}

	pub fn get_max_health(&self) -> f32 {
		self.max_health
	}
}
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::time::TimeCounter;

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Movement {
	pub speed: f32,
}

#[derive(Component)]
pub struct Shooting {
	pub cooldown: Timer,
}

pub struct ShootEvent(pub Vec2);

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Health {
	health: f32,
	max_health: f32,
}

impl Health {
	pub fn new(max_health: f32) -> Self {
		Self {
			health: max_health,
			max_health,
		}
	}

	/// # Returns
	/// True if the health reached zero.
	#[warn(unused_must_use)]
	#[must_use]
	pub fn take_damage(&mut self, amount: f32) -> bool {
		self.health -= amount;

		self.health <= 0.0
	}

	#[allow(unused)]
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
	#[allow(unused)]
	pub fn set_health(&mut self, hp: f32) {
		self.health = hp;
	}
}

pub trait Effect {
	fn apply(&self, movement: &mut Movement, health: &mut Health, shooting: &mut Shooting, time: &mut TimeCounter);
	fn finish(&self, movement: &mut Movement, health: &mut Health, shooting: &mut Shooting, time: &mut TimeCounter);
}

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Inventory {
	small_powerups: usize,
	big_powerups: usize,
}

#[allow(dead_code)]
impl Inventory {
	pub fn new() -> Self {
		Self {
			small_powerups: 1,
			big_powerups: 0,
		}
	}

	pub fn get_small_powerup_quantity(&self) -> usize {
		self.small_powerups
	}

	pub fn subtract_small_powerup(&mut self, amount: usize) -> bool {
		if amount > self.small_powerups {
			false
		} else {
			self.small_powerups -= amount;
			true
		}
	}

	pub fn add_small_powerup(&mut self, amount: usize) {
		self.small_powerups += amount;
	}

	pub fn get_big_powerup_quantity(&self) -> usize {
		self.big_powerups
	}

	pub fn subtract_big_powerup(&mut self, amount: usize) -> bool {
		if amount > self.big_powerups {
			false
		} else {
			self.big_powerups -= amount;
			true
		}
	}

	pub fn add_big_powerup(&mut self, amount: usize) {
		self.big_powerups += amount;
	}
}

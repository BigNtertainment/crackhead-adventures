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

	pub fn set_health(&mut self, h:f32) {
		self.health = h;
	}
}


#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Inventory {
	small_powerup: usize,
	small_powerup_health: f32,
	big_powerup: usize,
	big_powerup_health: f32,
}

impl Inventory {
	pub fn new(_small_powerup_health: f32, _big_powerup_health: f32) -> Self {
		Self {small_powerup: 2, big_powerup: 2, small_powerup_health: _small_powerup_health, big_powerup_health: _big_powerup_health}
	}


	pub fn get_small_powerup(&self) -> usize{
		self.small_powerup
	}

	pub fn get_small_powerup_health(&self) -> f32{
		self.small_powerup_health
	}

	pub fn subtract_small_powerup(&mut self, amount: usize) -> bool {
		if amount > self.small_powerup {
			return false;
		} else {
			self.small_powerup -= amount;
			return true;
		}
	}

	pub fn add_small_powerup(&mut self, amount: usize) {
		self.small_powerup += amount;
	}

	pub fn get_big_powerup(&self) -> usize{
		self.big_powerup
	}

	pub fn get_big_powerup_health(&self) -> f32{
		self.big_powerup_health
	}

	pub fn subtract_big_powerup(&mut self, amount: usize) -> bool {
		if amount > self.big_powerup {
			return false;
		} else {
			self.big_powerup -= amount;
			return true;
		}
	}

	pub fn add_big_powerup(&mut self, amount: usize) {
		self.big_powerup += amount;
	}
}
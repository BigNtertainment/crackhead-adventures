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
	pub health: f32,
}
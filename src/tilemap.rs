use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(load_level);
	}
}

fn load_level(mut commands: Commands) {
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::rgb(0.75, 0.25, 0.25),
			custom_size: Some(Vec2::splat(TILE_SIZE)),
			..Default::default()
		},
		transform: Transform::from_xyz(100.0, -50.0, 0.0),
		..Default::default()
	})
	.insert(TileCollider);
}
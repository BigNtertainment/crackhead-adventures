use bevy::prelude::*;

use crate::TILE_SIZE;
use crate::tilemap::Tile;
use crate::unit::Movement;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
	#[bundle]
	sprite_budle: SpriteBundle,
	name: Name,
	enemy: Enemy,
	movement: Movement,
}

impl Default for EnemyBundle {
	fn default() -> Self {
		Self {
			sprite_budle: SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(250.0 / 255.0, 44.0 / 255.0, 12.0 / 255.0),
					custom_size: Some(Vec2::splat(TILE_SIZE)),
					..Default::default()
				},
				..Default::default()
			},
			name: Name::new("Enemy"),
			enemy: Enemy,
			movement: Movement { speed: 10.0 }
		}
	}
}

impl Tile for EnemyBundle {
	fn at(position: Vec2) -> Self {
		Self {
			sprite_budle: SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(250.0 / 255.0, 44.0 / 255.0, 12.0 / 255.0),
					custom_size: Some(Vec2::splat(TILE_SIZE)),
					..Default::default()
				},
				transform: Transform::from_xyz(position.x, position.y, 0.0),
				..Default::default()
			},
			..Default::default()
		}
	}
}
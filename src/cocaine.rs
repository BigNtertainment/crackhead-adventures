use bevy::prelude::*;

use crate::tilemap::Tile;

#[derive(Component, Default)]
pub struct Cocaine;

#[derive(Bundle, Default)]
pub struct CocaineBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	cocaine: Cocaine,
}

impl Tile for CocaineBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				transform: Transform::from_translation(position.extend(25.0)),
				sprite: Sprite {
					flip_x,
					flip_y,
					..Default::default()
				},
				texture,
				..Default::default()
			},
			..Default::default()
		}
	}
}
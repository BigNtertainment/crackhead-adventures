use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;
use crate::tilemap::{TileCollider, Tile};

#[derive(Component)]
pub struct Player;

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Movement {
	speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.register_type::<Movement>()
			.add_system(player_movement.label("player_movement"))
			.add_system(camera_follow.after("player_movement"));
	}
}

#[derive(Bundle)]
pub struct PlayerBundle {
	#[bundle]
	sprite_budle: SpriteBundle,
	name: Name,
	player: Player,
	movement: Movement,
}

impl Default for PlayerBundle {
	fn default() -> Self {
		Self {
			sprite_budle: SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(0.25, 0.25, 0.75),
					custom_size: Some(Vec2::splat(TILE_SIZE)),
					..Default::default()
				},
				..Default::default()
			},
			name: Name::new("Player"),
			player: Player,
			movement: Movement { speed: 10.0 }
		}
	}
}

impl Tile for PlayerBundle {
	fn at(position: Vec2) -> Self {
		Self {
			sprite_budle: SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(0.25, 0.25, 0.75),
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

fn player_movement(
	mut player_query: Query<(&Movement, &mut Transform, &Sprite), With<Player>>,
	wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>
) {
	let (movement, mut transform, sprite) = player_query.iter_mut().next().expect("Player not found in the scene!");

	let mut direction = Vec3::new(0.0, 0.0, 0.0);

	if keyboard.pressed(KeyCode::W) {
		direction.y += 1.0;
	}

	if keyboard.pressed(KeyCode::S) {
		direction.y -= 1.0;
	}
	
	if keyboard.pressed(KeyCode::D) {
		direction.x += 1.0;
	}

	if keyboard.pressed(KeyCode::A) {
		direction.x -= 1.0;
	}

	if direction.length() != 0.0 {
		let mut target = transform.translation + direction.normalize() * movement.speed * TILE_SIZE * time.delta_seconds();

		let player_size = if let Some(player_size) = sprite.custom_size {
			Vec2::new(
				player_size.x * transform.scale.x,
				player_size.y * transform.scale.y,
			)
		} else {
			Vec2::new(transform.scale.x, transform.scale.y)
		};

		for wall_transform in wall_query.iter() {
			let collision = collide(
				target,
				player_size,
				wall_transform.translation,
				Vec2::splat(TILE_SIZE)
			);

			if let Some(collision) = collision {
				match collision {
					Collision::Bottom => {
						target.y = wall_transform.translation.y - TILE_SIZE;
					},
					Collision::Top => {
						target.y = wall_transform.translation.y + TILE_SIZE;
					},
					Collision::Left => {
						target.x = wall_transform.translation.x - TILE_SIZE;
					},
					Collision::Right => {
						target.x = wall_transform.translation.x + TILE_SIZE;
					},
					Collision::Inside => { /* what */ }
				};
			}
		}

		transform.translation = target;
	}
}

fn camera_follow(
	player_query: Query<&Transform, With<Player>>,
	mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
	let player_transform = player_query.single();
	let mut camera_transform = camera_query.single_mut();

	camera_transform.translation.x = player_transform.translation.x;
	camera_transform.translation.y = player_transform.translation.y;
}
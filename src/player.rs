use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;

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
			.add_startup_system(spawn_player)
			.add_system(player_movement);
	}
}

fn spawn_player(mut commands: Commands) {
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::rgb(0.25, 0.25, 0.75),
			custom_size: Some(Vec2::splat(TILE_SIZE)),
			..Default::default()
		},
		..Default::default()
	})
	.insert(Name::new("Player"))
	.insert(Player)
	.insert(Movement { speed: 10.0 });
}

fn player_movement(
	mut player_query: Query<(&Movement, &mut Transform), With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>
) {
	let (movement, mut transform) = player_query.single_mut();

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
		transform.translation += direction.normalize() * movement.speed * TILE_SIZE * time.delta_seconds();
	}
}
use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion},
    prelude::*,
    window::CursorMoved,
};
use bevy_inspector_egui::Inspectable;


use crate::TILE_SIZE;
use crate::HEIGHT;
use crate::WIDTH;

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
			.add_system(player_movement)
			.add_system(player_aim);
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

fn player_aim(
	mut player_query: Query<&mut Transform, (With<Player>, Without<Camera2d>)>,
	camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Player>)>,
    // mut cursor_event_reader: EventReader<CursorMoved>,
	window: Res<Windows>
) {
	let mut player_transform = player_query.single_mut();
	let (camera, camera_transform) = camera_query.single();

	// this is awful and it should not be mutable but i have no idea how to get that outside of the loops scope
	

	// this could be its own function
	// changing screen pos of cursor to world pos
	// still needs to check if the cursor is in the screen
	if let Some(target) = window.iter().next().unwrap().cursor_position(){
		let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

		let ndc = (target / window_size) * 2.0 - Vec2::ONE;

		println!("{}", ndc);

	// 	let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

	// 	let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
	// 	let world_pos: Vec2 = world_pos.truncate();

	// 	println!("World coords: {}/{}", world_pos.x, world_pos.y);
	// }
		let pos = player_transform.translation.truncate();

	
		let angle = (Vec2::Y).angle_between(ndc);
	// println!("{}", angle * 180.0 / PI);
	player_transform.rotation = Quat::from_rotation_z(angle);
	}

}
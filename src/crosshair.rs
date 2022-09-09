use bevy::prelude::*;

use crate::{GameState, HEIGHT, WIDTH, post_processing::MainCamera};

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(load_crosshair_sprite)
			.add_system_set(SystemSet::on_enter(GameState::Game).with_system(crosshair_setup))
			.add_system_set(SystemSet::on_update(GameState::Game).with_system(crosshair_update.after("camera_follow_player")))
			.add_system_set(SystemSet::on_exit(GameState::Game).with_system(crosshair_drop));
	}
}

#[derive(Component)]
pub struct Crosshair;

#[derive(Deref, DerefMut)]
struct CrosshairSprite(Handle<Image>);

fn load_crosshair_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(CrosshairSprite(asset_server.load("./img/crosshair.png")));
}

fn crosshair_setup(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
	sprite: Res<CrosshairSprite>,
) {
	let window = windows.get_primary_mut().unwrap();

	window.set_cursor_visibility(false);

	commands
		.spawn_bundle(SpriteBundle {
			texture: sprite.clone(),
			..Default::default()
		})
		.insert(Crosshair)
		.insert(Name::new("Crosshair"));
}

fn crosshair_update(
	mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
	camera_query: Query<&Transform, (With<MainCamera>, Without<Crosshair>)>,
	windows: Res<Windows>,
) {
	let mut crosshair_transform = crosshair_query.single_mut();
	let camera_transform = camera_query.single();
	let window = windows.get_primary().unwrap();

	if let Some(cursor_position) = window.cursor_position() {
		crosshair_transform.translation = camera_transform.translation
			- Vec3::new(WIDTH, HEIGHT, 0.0) / 2.0
			+ cursor_position.extend(0.0);

			crosshair_transform.translation.z = 100.0;
	}
}

fn crosshair_drop(
	mut commands: Commands,
	crosshair_query: Query<Entity, With<Crosshair>>,
	mut windows: ResMut<Windows>,
) {
	let crosshair = crosshair_query.single();
	let window = windows.get_primary_mut().unwrap();

	window.set_cursor_visibility(true);

	commands.entity(crosshair).despawn_recursive();
}

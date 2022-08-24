use std::time::Duration;

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use bevy_kira_audio::prelude::*;

use rand::prelude::*;

use crate::bullet::{Bullet, BulletBundle, BulletTexture, ShotEvent};
use crate::cocaine::Cocaine;
use crate::enemy::Enemy;
use crate::post_processing::{
	update_post_processing_effects, CameraRenderImage, DefaultMaterial, MainCamera,
	PostProcessingLayer, ScreenRes,
};
use crate::tilemap::{Tile, Tilemap};
use crate::unit::{Health, Inventory, Movement, Shooting};
use crate::HEIGHT;
use crate::WIDTH;
use crate::{GameState, TILE_SIZE};

mod effect;
mod post_processing;
mod ui;

use ui::{drop_ui, load_font, ui_setup, update_ui};

use self::effect::{BigPowerup, EffectData, SmallPowerup};
use self::post_processing::{
	clean_post_processing, BigPowerupMaterial,
	PlayerPostProcessingPlugin, SmallPowerupMaterial,
};

pub const WEAPON_COOLDOWN: f32 = 0.5;
pub const SMALL_POWERUP_DURATION: f32 = 5.0;
pub const BIG_POWERUP_DURATION: f32 = 5.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(PlayerPostProcessingPlugin)
			.register_type::<Movement>()
			.add_startup_system(load_shot_sound)
			.add_startup_system(load_font)
			.add_system_set(SystemSet::on_enter(GameState::Game).with_system(ui_setup))
			.add_system_set(SystemSet::on_exit(GameState::Game).with_system(drop_ui))
			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(player_movement.label("player_movement"))
					.with_system(
						camera_follow
							.label("camera_follow_player")
							.after("player_movement"),
					)
					.with_system(player_aim.label("player_aim").after("player_movement"))
					.with_system(player_shoot.after("player_aim"))
					.with_system(damage_yourself)
					.with_system(get_shot)
					.with_system(update_ui)
					.with_system(pick_up_cocaine)
					.with_system(craft_magic_dust)
					.with_system(use_powerup),
			);
	}
}

#[derive(Bundle)]
pub struct PlayerBundle {
	#[bundle]
	sprite_budle: SpriteBundle,
	name: Name,
	player: Player,
	movement: Movement,
	health: Health,
	shooting: Shooting,
	rapier_collider: Collider,
	inventory: Inventory,
	effect: EffectData,
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
			movement: Movement { speed: 10.0 },
			health: Health::new(100.0),
			shooting: Shooting {
				cooldown: Timer::new(Duration::from_secs_f32(WEAPON_COOLDOWN), false),
			},
			inventory: Inventory::new(),
			effect: EffectData {
				effect: None,
				duration: Timer::from_seconds(0.0, false),
			},
			rapier_collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
		}
	}
}

impl Tile for PlayerBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_budle: SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::splat(TILE_SIZE)),
					flip_x,
					flip_y,
					..Default::default()
				},
				texture,
				transform: Transform::from_xyz(position.x, position.y, 50.0),
				..Default::default()
			},
			..Default::default()
		}
	}
}

fn player_movement(
	mut player_query: Query<(Entity, &Movement, &mut Transform, &Collider), With<Player>>,
	enemy_query: Query<Entity, (With<Enemy>, Without<Player>)>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	rapier_context: Res<RapierContext>,
) {
	let (player_entity, movement, mut transform, rapier_collider) = player_query
		.iter_mut()
		.next()
		.expect("Player not found in the scene!");

	let enemies: Vec<Entity> = enemy_query.iter().collect();

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
		let shape = rapier_collider;
		let position = transform.translation.truncate();
		let rotation = 0.0; // transform.rotation.z;
		let direction = direction.normalize().truncate();
		let max_time_of_impact = movement.speed * TILE_SIZE * time.delta_seconds();

		let predicate = |entity| !enemies.contains(&entity);

		let filter = QueryFilter::default()
			.exclude_collider(player_entity)
			.predicate(&predicate);

		let movement_vector = Vec2::new(
			if let Some((_, hit)) = rapier_context.cast_shape(
				position,
				rotation,
				direction * Vec2::new(1.0, 0.0),
				&shape,
				max_time_of_impact,
				filter,
			) {
				direction * (hit.toi - 0.1)
			} else {
				direction * (max_time_of_impact - 0.1)
			}
			.x,
			if let Some((_, hit)) = rapier_context.cast_shape(
				position,
				rotation,
				direction * Vec2::new(0.0, 1.0),
				&shape,
				max_time_of_impact,
				filter,
			) {
				direction * (hit.toi - 0.05)
			} else {
				direction * (max_time_of_impact - 0.05)
			}
			.y,
		);

		transform.translation += movement_vector.extend(0.0);
	}
}

fn camera_follow(
	player_query: Query<&Transform, With<Player>>,
	mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
) {
	let mut camera_transform = camera_query.single_mut();
	let player_transform = player_query.single();

	camera_transform.translation.x = player_transform.translation.x;
	camera_transform.translation.y = player_transform.translation.y;
}

fn damage_yourself(
	mut player_query: Query<&mut Health, With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut player_health = player_query.single_mut();

	if cfg!(debug_assertions) && keyboard.just_pressed(KeyCode::Space) {
		if player_health.take_damage(rand::thread_rng().gen::<f32>() * 10.0 + 10.0) {
			state
				.set(GameState::GameOver)
				.expect("Failed to change states");
		}
	}
}

fn player_aim(mut player_query: Query<&mut Transform, With<Player>>, window: Res<Windows>) {
	let mut player_transform = player_query.single_mut();

	if let Some(target) = window.iter().next().unwrap().cursor_position() {
		let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

		let target = target - window_size / 2.0;

		let angle = Vec2::Y.angle_between(target);
		player_transform.rotation = Quat::from_rotation_z(angle);
	}
}

fn load_shot_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
	let sound = asset_server.load("shot.wav");

	commands.insert_resource(ShotSound(sound));
}
struct ShotSound(Handle<AudioSource>);

fn player_shoot(
	mut commands: Commands,
	mut player_query: Query<(&Transform, &mut Shooting), With<Player>>,
	world_query: Query<Entity, With<Tilemap>>,
	buttons: Res<Input<MouseButton>>,
	time: Res<Time>,
	audio: Res<Audio>,
	shot_sound: Res<ShotSound>,
	bullet_texture: Res<BulletTexture>,
) {
	let (player_transform, mut shooting) = player_query.single_mut();
	let world = world_query.single();

	shooting.cooldown.tick(time.delta());

	if !shooting.cooldown.finished() {
		return;
	}

	if buttons.just_pressed(MouseButton::Left) {
		// Spawn the bullets
		let mut bullets = Vec::new();

		for i in 1..5 {
			let mut bullet_transform = player_transform
				.with_translation(player_transform.translation + player_transform.up() * TILE_SIZE);

			bullet_transform.rotate_z((i - 2) as f32 * (0.02 + random::<f32>() * 0.01));

			bullets.push(
				commands
					.spawn_bundle(BulletBundle {
						sprite_bundle: SpriteBundle {
							transform: bullet_transform,
							texture: bullet_texture.clone(),
							..Default::default()
						},
						bullet: Bullet { speed: 2000.0 },
						..Default::default()
					})
					.id(),
			);
		}

		commands.entity(world).push_children(&bullets);

		audio.play(shot_sound.0.clone()).with_volume(0.15);

		// Reset the cooldown timer
		shooting.cooldown.reset();
	}
}

fn use_powerup(
	mut commands: Commands,
	mut player_query: Query<
		(&mut Inventory, &mut Movement, &mut Health, &mut EffectData),
		With<Player>,
	>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	post_processing_pass_layer: Res<PostProcessingLayer>,
	screen: Res<ScreenRes>,
	source_image: Res<CameraRenderImage>,
	mut default_materials: ResMut<Assets<DefaultMaterial>>,
	mut small_powerup_materials: ResMut<Assets<SmallPowerupMaterial>>,
	mut big_powerup_materials: ResMut<Assets<BigPowerupMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let (mut inventory, mut movement, mut health, mut effect_data) = player_query.single_mut();

	effect_data.duration.tick(time.delta());

	if effect_data.duration.just_finished() {
		// Remove the effects
		effect_data.finish(movement.as_mut(), health.as_mut());
		clean_post_processing(
			&mut commands,
			&screen.0,
			&mut meshes,
			&post_processing_pass_layer,
			&source_image,
			&mut default_materials,
		);
	}

	// Don't take more drugs if you're high already
	if effect_data.effect.is_some() {
		return;
	}

	// Small powerup is under E
	if keyboard.just_pressed(KeyCode::E) && inventory.subtract_small_powerup(1) {
		effect_data.apply(
			Some(Box::new(SmallPowerup)),
			movement.as_mut(),
			health.as_mut(),
			SMALL_POWERUP_DURATION,
		);

		// Add a post-processing effect
		update_post_processing_effects(
			&mut commands,
			&screen,
			small_powerup_materials.add(SmallPowerupMaterial {
				source_image: source_image.0.clone(),
			}),
			&mut meshes,
			&post_processing_pass_layer,
		);
	}
	// Big powerup is under R
	else if keyboard.just_pressed(KeyCode::R) && inventory.subtract_big_powerup(1) {
		effect_data.apply(
			Some(Box::new(BigPowerup)),
			movement.as_mut(),
			health.as_mut(),
			BIG_POWERUP_DURATION,
		);

		// Add a post-processing effect
		update_post_processing_effects(
			&mut commands,
			&screen,
			big_powerup_materials.add(BigPowerupMaterial {
				source_image: source_image.0.clone(),
			}),
			&mut meshes,
			&post_processing_pass_layer,
		);
	}
}

fn pick_up_cocaine(
	mut commands: Commands,
	mut player_query: Query<(&mut Inventory, &Transform), With<Player>>,
	cocaine_query: Query<(Entity, &Transform), With<Cocaine>>,
) {
	let (mut player_inventory, player_transform) = player_query.single_mut();

	for (cocaine, cocaine_transform) in cocaine_query.iter() {
		if (player_transform.translation.truncate() - cocaine_transform.translation.truncate())
			.length()
			<= TILE_SIZE / 2.0
		{
			player_inventory.add_small_powerup(1);
			commands.entity(cocaine).despawn_recursive();
		}
	}
}

fn craft_magic_dust(
	mut player_query: Query<&mut Inventory, With<Player>>,
	keyboard: Res<Input<KeyCode>>,
) {
	let mut inventory = player_query.single_mut();

	// Press T to craft magic dust
	if keyboard.just_pressed(KeyCode::T) {
		if inventory.subtract_small_powerup(3) {
			inventory.add_big_powerup(1);
		}
	}
}

fn get_shot(
	mut player_query: Query<(Entity, &mut Health), With<Player>>,
	mut shot_events: EventReader<ShotEvent>,
	mut state: ResMut<State<GameState>>,
) {
	let (player, mut health) = player_query.single_mut();

	for shot in shot_events.iter() {
		let entity = shot.0;

		if entity != player {
			continue;
		}

		if health.take_damage(25.0 + random::<f32>() * 10.0) {
			if state.set(GameState::GameOver).is_err() {}
		}
	}
}

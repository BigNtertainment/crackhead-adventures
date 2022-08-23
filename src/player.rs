use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use bevy_kira_audio::prelude::*;

use rand::prelude::*;
use rand::seq::SliceRandom;

use crate::cocaine::Cocaine;
use crate::enemy::{Enemy, EnemyBodyBundle, EnemyTextures};
use crate::tilemap::{Tile, Tilemap};
use crate::unit::{Effect, Health, Inventory, Movement, Shooting};
use crate::HEIGHT;
use crate::WIDTH;
use crate::{GameState, TILE_SIZE};

mod ui;

use ui::{drop_ui, ui_setup, update_ui};

pub const WEAPON_RANGE: f32 = 400.0;
pub const WEAPON_COOLDOWN: f32 = 0.5;
pub const SMALL_POWERUP_DURATION: f32 = 5.0;
pub const BIG_POWERUP_DURATION: f32 = 5.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<Movement>()
			.add_startup_system(load_shot_sound)
			.add_system_set(SystemSet::on_enter(GameState::Game).with_system(ui_setup))
			.add_system_set(SystemSet::on_exit(GameState::Game).with_system(drop_ui))
			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(player_movement.label("player_movement"))
					.with_system(camera_follow.label("camera_follow_player").after("player_movement"))
					.with_system(player_aim.label("player_aim").after("player_movement"))
					.with_system(player_shoot.after("player_aim"))
					.with_system(damage_yourself)
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
			rapier_collider: Collider::ball(TILE_SIZE / 2.0),
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
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	rapier_context: Res<RapierContext>,
) {
	let (player_entity, movement, mut transform, rapier_collider) = player_query
		.iter_mut()
		.next()
		.expect("Player not found in the scene!");

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
		let filter = QueryFilter::default().exclude_collider(player_entity);

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
	mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
	let player_transform = player_query.single();
	let mut camera_transform = camera_query.single_mut();

	camera_transform.translation.x = player_transform.translation.x;
	camera_transform.translation.y = player_transform.translation.y;
}

fn damage_yourself(
	mut player_query: Query<&mut Health, With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>,
) {
	let mut player_health = player_query.single_mut();

	if keyboard.just_pressed(KeyCode::Space) {
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
	mut player_query: Query<(Entity, &Transform, &mut Shooting), With<Player>>,
	enemies_query: Query<(Entity, &Transform), With<Enemy>>,
	world_query: Query<Entity, With<Tilemap>>,
	rapier_context: Res<RapierContext>,
	buttons: Res<Input<MouseButton>>,
	time: Res<Time>,
	window: Res<Windows>,
	audio: Res<Audio>,
	shot_sound: Res<ShotSound>,
	enemy_textures: Res<EnemyTextures>,
) {
	let (player_entity, player_transform, mut shooting) = player_query.single_mut();
	let world = world_query.single();

	shooting.cooldown.tick(time.delta());

	if !shooting.cooldown.finished() {
		return;
	}

	let window_size = Vec2::new(WIDTH, HEIGHT);

	if let Some(target) = window.iter().next().unwrap().cursor_position() {
		let target = target * window.iter().next().unwrap().scale_factor() as f32;
		let target = target - window_size / 2.0;

		let ray_origin = player_transform.translation.truncate();
		let ray_direction = target.normalize();
		let max_time_of_impact = WEAPON_RANGE;
		let solid = true;
		let filter = QueryFilter::default().exclude_collider(player_entity);

		if buttons.just_pressed(MouseButton::Left) {
			if let Some((entity, _toi)) = rapier_context.cast_ray(
				ray_origin,
				ray_direction,
				max_time_of_impact,
				solid,
				filter,
			) {
				for (enemy, enemy_transform) in enemies_query.iter() {
					if entity.id() == enemy.id() {
						commands.entity(entity).despawn_recursive();

						// Spawn the enemy body
						let body = commands.spawn_bundle(EnemyBodyBundle {
							sprite_bundle: SpriteBundle {
								transform: Transform::from_translation(enemy_transform.translation)
									.with_rotation(Quat::from_rotation_z(
										rand::random::<f32>() * 2.0 * PI,
									)),
								texture: enemy_textures.body.clone(),
								..Default::default()
							},
						}).id();

						commands.entity(world).push_children(&[body]);

						// Spawn a few blood splatters
						let temp: Vec<u32> = (0..4).collect();

						let mut splatters = Vec::new();

						for _ in 0..(*temp.choose(&mut rand::thread_rng()).unwrap()) {
							splatters.push(commands.spawn_bundle(EnemyBodyBundle {
								sprite_bundle: SpriteBundle {
									transform: Transform::from_translation(
										enemy_transform.translation
											+ Vec3::new(
												rand::random::<f32>() * 60.0 - 30.0,
												rand::random::<f32>() * 60.0 - 30.0,
												-10.0,
											),
									)
									.with_rotation(
										Quat::from_rotation_z(rand::random::<f32>() * 2.0 * PI),
									),
									texture: enemy_textures.blood_splatter.clone(),
									..Default::default()
								},
							}).id());
						}

						commands.entity(world).push_children(&splatters);
					}
				}
			}

			audio.play(shot_sound.0.clone()).with_volume(0.15);

			// Reset the cooldown timer
			shooting.cooldown.reset();
		}
	}
}

#[derive(Component)]
pub struct EffectData {
	effect: Option<Box<dyn Effect + Send + Sync>>,
	duration: Timer,
}

impl EffectData {
	fn apply(
		&mut self,
		effect: Option<Box<dyn Effect + Send + Sync>>,
		movement: &mut Movement,
		health: &mut Health,
		duration: f32,
	) {
		self.effect = effect;

		if let Some(effect) = &mut self.effect {
			effect.apply(movement, health);
		}

		self.duration = Timer::from_seconds(duration, false);
	}

	fn finish(&mut self, movement: &mut Movement, health: &mut Health) {
		if let Some(effect) = &self.effect {
			effect.finish(movement, health);
		}

		self.effect = None;
	}
}

pub struct SmallPowerup;

impl Effect for SmallPowerup {
	fn apply(&self, movement: &mut Movement, health: &mut Health) {
		movement.speed *= 2.0;
		health.heal(20.0);
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health) {
		movement.speed /= 2.0;
	}
}

pub struct BigPowerup;

impl Effect for BigPowerup {
	fn apply(&self, movement: &mut Movement, health: &mut Health) {
		movement.speed *= 5.0;
		if health.get_health() > 5.0 {
			health.set_health(5.0);
		}
	}

	fn finish(&self, movement: &mut Movement, _: &mut Health) {
		movement.speed /= 5.0;
	}
}

fn use_powerup(
	mut player_query: Query<
		(&mut Inventory, &mut Movement, &mut Health, &mut EffectData),
		With<Player>,
	>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (mut inventory, mut movement, mut health, mut effect_data) = player_query.single_mut();

	effect_data.duration.tick(time.delta());

	if effect_data.duration.just_finished() {
		// Remove the effects
		effect_data.finish(movement.as_mut(), health.as_mut());
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
	}
	// Big powerup is under R
	else if keyboard.just_pressed(KeyCode::R) && inventory.subtract_big_powerup(1) {
		effect_data.apply(
			Some(Box::new(BigPowerup)),
			movement.as_mut(),
			health.as_mut(),
			BIG_POWERUP_DURATION,
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

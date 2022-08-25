use std::f32::consts::PI;
use std::path::PathBuf;
use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use navmesh::NavVec3;
use rand::random;
use rand::seq::SliceRandom;

use crate::audio::{EnemyShotSound, Screams};
use crate::bullet::{Bullet, BulletBundle, BulletTexture, ShotEvent, BULLET_COLLIDER_WIDTH, BULLET_COLLIDER_HEIGHT};
use crate::enemy_nav_mesh::EnemyNavMesh;
use crate::player::Player;
use crate::post_processing::MainCamera;
use crate::tilemap::{TexturesMemo, Tile, Tilemap};
use crate::unit::{Movement, ShootEvent, Shooting};
use crate::{GameState, TILE_SIZE};

pub const ENEMY_SIGHT: f32 = 12.0 * TILE_SIZE;
pub const ENEMY_HEARING: f32 = 10.0 * TILE_SIZE;
pub const SHOCK_DURATION: f32 = 0.5;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(load_enemy_textures).add_system_set(
			SystemSet::on_update(GameState::Game)
				.with_system(update_enemy_ai.label("update_enemy_ai"))
				.with_system(alert_on_shot_sound)
				.with_system(update_enemy_position.after("update_enemy_ai"))
				.with_system(update_enemy_texture)
				.with_system(get_shot),
		);
	}
}

#[derive(Component)]
pub struct Enemy {
	ai_state: EnemyAiState,
	shock_timer: Timer,
}

#[derive(Bundle)]
pub struct EnemyBundle {
	#[bundle]
	sprite_budle: SpriteBundle,
	name: Name,
	enemy: Enemy,
	movement: Movement,
	shooting: Shooting,
	rapier_collider: Collider,
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
			enemy: Enemy {
				ai_state: EnemyAiState::Idle,
				shock_timer: Timer::new(Duration::from_secs_f32(SHOCK_DURATION), false),
			},
			movement: Movement { speed: 3.0 },
			shooting: Shooting {
				cooldown: Timer::from_seconds(1.0, false),
			},
			rapier_collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
		}
	}
}

impl Tile for EnemyBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_budle: SpriteBundle {
				transform: Transform::from_xyz(position.x, position.y, 45.0),
				texture,
				sprite: Sprite {
					flip_x,
					flip_y,
					..Default::default()
				},
				..Default::default()
			},
			..Default::default()
		}
	}
}

#[derive(Bundle)]
pub struct EnemyBodyBundle {
	#[bundle]
	pub sprite_bundle: SpriteBundle,
}

fn load_enemy_textures(
	mut commands: Commands,
	mut textures: ResMut<TexturesMemo>,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(EnemyTextures {
		idle: textures.get(&PathBuf::from("img/enemy_idle.png"), &asset_server),
		active: textures.get(&PathBuf::from("img/enemy.png"), &asset_server),
		body: textures.get(&PathBuf::from("img/enemy_ded.png"), &asset_server),
		blood_splatter: textures.get(&PathBuf::from("img/blood_splatter.png"), &asset_server),
	});
}

pub struct EnemyTextures {
	pub idle: Handle<Image>,
	pub active: Handle<Image>,
	pub body: Handle<Image>,
	pub blood_splatter: Handle<Image>,
}

enum EnemyAiState {
	Idle,
	Alert {
		path: Option<Vec<NavVec3>>,
		current: usize,
	},
	Combat {
		player_position: Vec2,
	},
}

fn is_on_screen(point: Vec2, window: &Window, camera: &Transform) -> bool {
	let screen_position = point - camera.translation.truncate();

	screen_position.x.abs() < (window.width() - TILE_SIZE) / 2.0
		&& screen_position.y.abs() < (window.height() - TILE_SIZE) / 2.0
}

fn update_enemy_ai(
	mut commands: Commands,
	mut enemies: Query<(Entity, &mut Transform, &mut Shooting, &mut Enemy)>,
	mut player: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
	tilemap: Query<Entity, (With<Tilemap>, Without<Player>, Without<Enemy>)>,
	camera: Query<
		&Transform,
		(
			With<MainCamera>,
			Without<Player>,
			Without<Enemy>,
			Without<Tilemap>,
		),
	>,
	mut shoot_event: EventWriter<ShootEvent>,
	rapier_context: Res<RapierContext>,
	time: Res<Time>,
	windows: Res<Windows>,
	nav_mesh: Res<EnemyNavMesh>,
	audio: Res<Audio>,
	shot_sound: Res<EnemyShotSound>,
	bullet_texture: Res<BulletTexture>,
) {
	let (player, player_transform) = player.single_mut();
	let tilemap = tilemap.single();
	let camera = camera.single();

	let player_position = player_transform.translation.truncate();

	for (entity, mut transform, mut shooting, mut enemy) in enemies.iter_mut() {
		shooting.cooldown.tick(time.delta());

		let position = transform.translation.truncate();

		// Look if there is a direct line of sight to the player
		let shape = Collider::cuboid(BULLET_COLLIDER_WIDTH, BULLET_COLLIDER_HEIGHT);
		let shape_origin = position;
		let shape_direction = (player_position - position).normalize();
		let shape_rotation = transform.rotation.z;
		let max_time_of_impact = ENEMY_SIGHT;
		let filter = QueryFilter::default()
			.exclude_collider(entity)
			.exclude_sensors();

		if let Some((entity, _)) = rapier_context.cast_shape(
			shape_origin,
			shape_rotation,
			shape_direction,
			&shape,
			max_time_of_impact,
			filter,
		) {
			if entity.id() == player.id() {
				// The enemy can see the player
				enemy.ai_state = EnemyAiState::Combat { player_position };

				transform.rotation =
					Quat::from_rotation_z(Vec2::Y.angle_between(player_position - position));

				// Don't shoot when off-screen
				if is_on_screen(position, windows.primary(), camera) {
					// Don't shoot immediately
					enemy.shock_timer.tick(time.delta());

					if enemy.shock_timer.finished() && shooting.cooldown.finished() {
						// Shoot at the player
						let mut bullet_transform = transform
							.with_translation(transform.translation + transform.up() * TILE_SIZE);

						bullet_transform.rotate_z(random::<f32>() * 0.05);

						let bullet = commands
							.spawn_bundle(BulletBundle {
								sprite_bundle: SpriteBundle {
									transform: bullet_transform,
									texture: bullet_texture.clone(),
									..Default::default()
								},
								bullet: Bullet { speed: 2000.0 },
								..Default::default()
							})
							.id();

						commands.entity(tilemap).push_children(&[bullet]);

						audio.play(shot_sound.clone()).with_volume(0.1);

						shoot_event.send(ShootEvent(position));

						shooting.cooldown.reset();
					}

					continue;
				}
			}

			// Not in combat
			if let EnemyAiState::Combat { player_position } = enemy.ai_state {
				// When exiting combat
				let path = nav_mesh
					.get_nav_mesh()
					.expect("The nav mesh has not been baked!")
					.find_path(
						transform.translation.to_array().into(),
						player_position.to_array().into(),
						navmesh::NavQuery::Closest,
						navmesh::NavPathMode::Accuracy,
					);

				enemy.ai_state = EnemyAiState::Alert { path, current: 0 };
			}

			enemy.shock_timer.reset();
		}
	}
}

fn alert_on_shot_sound(
	mut enemies: Query<(&Transform, &mut Enemy)>,
	mut shot_events: EventReader<ShootEvent>,
	nav_mesh: Res<EnemyNavMesh>,
) {
	for shot_event in shot_events.iter() {
		for (enemy_transform, mut enemy) in enemies.iter_mut() {
			let enemy_position = enemy_transform.translation.truncate();

			if (enemy_position - shot_event.0).length() <= ENEMY_HEARING {
				let path = nav_mesh
					.get_nav_mesh()
					.expect("The nav mesh has not been baked!")
					.find_path(
						enemy_position.to_array().into(),
						shot_event.0.to_array().into(),
						navmesh::NavQuery::Closest,
						navmesh::NavPathMode::Accuracy,
					);

				enemy.ai_state = EnemyAiState::Alert { path, current: 0 };
			}
		}
	}
}

fn update_enemy_position(
	mut enemies: Query<(&mut Transform, &mut Enemy, &Movement)>,
	time: Res<Time>,
) {
	for (mut transform, mut enemy, movement) in enemies.iter_mut() {
		if let EnemyAiState::Alert {
			path: Some(path),
			current,
		} = &mut enemy.ai_state
		{
			let mut target = path[*current];

			let mut movement_vector =
				Vec2::new(target.x, target.y) - transform.translation.truncate();

			while movement_vector.length() <= 5.0 {
				*current += 1;

				if *current == path.len() {
					enemy.ai_state = EnemyAiState::Idle;
					break;
				}

				target = path[*current];

				movement_vector = Vec2::new(target.x, target.y) - transform.translation.truncate();
			}

			let direction = movement_vector.normalize_or_zero();

			transform.translation +=
				(direction * TILE_SIZE * movement.speed * time.delta_seconds()).extend(0.0);
			transform.rotation = Quat::from_rotation_z(Vec2::Y.angle_between(direction));
		}
	}
}

fn update_enemy_texture(
	mut enemy_query: Query<(&mut Handle<Image>, &Enemy)>,
	textures: Res<EnemyTextures>,
) {
	for (mut enemy_texture, enemy) in enemy_query.iter_mut() {
		enemy_texture.clone_from(match enemy.ai_state {
			EnemyAiState::Idle => &textures.idle,
			EnemyAiState::Alert {
				path: _,
				current: _,
			}
			| EnemyAiState::Combat { player_position: _ } => &textures.active,
		});
	}
}

fn get_shot(
	world: &World,
	mut commands: Commands,
	tilemap_query: Query<Entity, With<Tilemap>>,
	enemy_query: Query<Entity, With<Enemy>>,
	mut shot_events: EventReader<ShotEvent>,
	enemy_textures: Res<EnemyTextures>,
	audio: Res<Audio>,
	screams: Res<Screams>,
) {
	let tilemap = tilemap_query.single();
	let mut enemies: Vec<Entity> = enemy_query.iter().collect();

	for shot in shot_events.iter() {
		let enemy = shot.0;

		if !enemies.contains(&enemy) {
			continue;
		}

		if let Some(enemy_transform) = world.get::<Transform>(enemy) {
			// Spawn the enemy body
			let body = commands
				.spawn_bundle(EnemyBodyBundle {
					sprite_bundle: SpriteBundle {
						transform: Transform::from_translation(enemy_transform.translation)
							.with_rotation(Quat::from_rotation_z(rand::random::<f32>() * 2.0 * PI)),
						texture: enemy_textures.body.clone(),
						..Default::default()
					},
				})
				.id();

			commands.entity(tilemap).push_children(&[body]);

			audio
				.play(
					screams
						.choose(&mut rand::thread_rng())
						.expect("No scream sounds found.")
						.clone(),
				)
				.with_volume(0.3);

			// Spawn a few blood splatters
			let temp: Vec<u32> = (0..4).collect();

			let mut splatters = Vec::new();

			for _ in 0..(*temp.choose(&mut rand::thread_rng()).unwrap()) {
				splatters.push(
					commands
						.spawn_bundle(EnemyBodyBundle {
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
						})
						.id(),
				);
			}

			commands.entity(tilemap).push_children(&splatters);

			commands.entity(enemy).despawn_recursive();

			let index = enemies.iter().position(|_enemy| *_enemy == enemy).unwrap();
			enemies.remove(index);
		}
	}
}

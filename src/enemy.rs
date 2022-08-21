use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioSource, Audio, AudioControl};
use bevy_rapier2d::prelude::*;
use navmesh::NavVec3;

use crate::enemy_nav_mesh::EnemyNavMesh;
use crate::player::Player;
use crate::{TILE_SIZE, GameState};
use crate::tilemap::Tile;
use crate::unit::{Movement, Shooting, Health};

pub const ENEMY_SIGHT: f32 = 400.0;
pub const SHOCK_DURATION: f32 = 1.25;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(load_shot_sound)

			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(update_enemy_ai)
			);
	}
}

#[derive(Component)]
pub struct Enemy {
	ai_state: EnemyAiState,
	shock_timer: Timer
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
				shock_timer: Timer::new(Duration::from_secs_f32(SHOCK_DURATION), false)
			},
			movement: Movement { speed: 3.0 },
			shooting: Shooting {
				cooldown: Timer::from_seconds(1.0, false)
			},
			rapier_collider: Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/2.0),
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

fn load_shot_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
	let sound = asset_server.load("shot.wav");

	commands.insert_resource(ShotSound(sound));
}

struct ShotSound(Handle<AudioSource>);

enum EnemyAiState {
	Idle,
	Alert { 
		path: Option<Vec<NavVec3>>,
		current: usize
	}
}

fn update_enemy_ai(
	mut enemies: Query<(Entity, &mut Transform, &Movement, &mut Shooting, &mut Enemy)>,
	mut player: Query<(Entity, &Transform, &mut Health), (With<Player>, Without<Enemy>)>,
	rapier_context: Res<RapierContext>,
	time: Res<Time>,
	mut state: ResMut<State<GameState>>,
	nav_mesh: Res<EnemyNavMesh>,
	audio: Res<Audio>,
	shot_sound: Res<ShotSound>
) {
	let (player, player_transform, mut player_health) = player.single_mut();

	let player_position = player_transform.translation;

	for (entity, mut transform, movement, mut shooting, mut enemy) in enemies.iter_mut() {
		shooting.cooldown.tick(time.delta());

		// Look if there is a direct line of sight to the player
		let ray_origin = transform.translation.truncate();
		let ray_direction = (player_position - transform.translation).truncate().normalize();
		let max_time_of_impact = ENEMY_SIGHT;
		let solid = true;
		let filter = QueryFilter::default()
			.exclude_collider(entity);

		if let Some((entity, _))  = rapier_context.cast_ray(
			ray_origin, ray_direction, max_time_of_impact, solid, filter
		) {
			if entity.id() == player.id() {
				// The enemy can see the player
				let path = nav_mesh.get_nav_mesh().expect("The nav mesh has not been baked!").find_path(
					transform.translation.to_array().into(),
					player_position.to_array().into(),
					navmesh::NavQuery::Closest,
					navmesh::NavPathMode::Accuracy
				);

				enemy.ai_state = EnemyAiState::Alert {
					path,
					current: 0
				};

				transform.rotation = Quat::from_rotation_z(
					Vec2::Y.angle_between(
						(player_position - transform.translation).truncate()
					)
				);

				// Don't shoot immediately
				enemy.shock_timer.tick(time.delta());

				if enemy.shock_timer.finished() && shooting.cooldown.finished() {
					if player_health.take_damage(rand::random::<f32>() * 5.0 + 20.0) {
						state.set(GameState::GameOver).expect("Failed to change state");
					}

					audio
						.play(shot_sound.0.clone())
						.with_volume(0.1);

					shooting.cooldown.reset();
				}

				continue;
			}
		}

		enemy.shock_timer.reset();

		if let EnemyAiState::Alert { path: Some(path), current } = &mut enemy.ai_state {
			let target = path[*current];

			let movement_vector = Vec2::new(target.x, target.y) - transform.translation.truncate();

			// If the enemy reached its destination
			if movement_vector.length() <= 5.0 {
				*current += 1;
				
				if *current == path.len() {
					enemy.ai_state = EnemyAiState::Idle;
				}
				
				continue;
			}

			let direction = movement_vector.normalize_or_zero();

			transform.translation += (direction * TILE_SIZE * movement.speed * time.delta_seconds()).extend(0.0);

			transform.rotation = Quat::from_rotation_z(
				Vec2::Y.angle_between(direction)
			);
		}
	}
}
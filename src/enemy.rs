use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::{TILE_SIZE, GameState};
use crate::tilemap::{Tile, TileCollider};
use crate::unit::Movement;

pub const ENEMY_SIGHT: f32 = 400.0;
pub const SHOCK_DURATION: f32 = 1.25;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
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
			movement: Movement { speed: 7.0 },
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

enum EnemyAiState {
	Idle,
	Alert(Vec2)
}

fn update_enemy_ai(
	mut enemies: Query<(Entity, &mut Transform, &Movement, &mut Enemy)>,
	player: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
	wall_query: Query<&Transform, (With<TileCollider>, Without<Player>, Without<Enemy>)>,
	rapier_context: Res<RapierContext>,
	time: Res<Time>,
	mut lines: ResMut<DebugLines>
) {
	let (player, player_transform) = player.single();

	let player_position = player_transform.translation;

	for (entity, mut transform, movement, mut enemy) in enemies.iter_mut() {
		// Look if there is a direct line of sight to the player
		let ray_origin = transform.translation.truncate();
		let ray_direction = (player_position - transform.translation).truncate().normalize();
		let max_time_of_impact = ENEMY_SIGHT;
		let solid = true;
		let filter = QueryFilter::default()
			.exclude_collider(entity);

		if cfg!(debug_assertions) {
			lines.line(ray_origin.extend(0.0), (ray_origin + ray_direction * max_time_of_impact).extend(0.0), 0.0);
		}

		if let Some((entity, _))  = rapier_context.cast_ray(
			ray_origin, ray_direction, max_time_of_impact, solid, filter
		) {
			if entity.id() == player.id() {
				// The enemy can see the player
				enemy.ai_state = EnemyAiState::Alert(player_position.truncate());

				// Don't shoot imidietally
				enemy.shock_timer.tick(time.delta());

				if enemy.shock_timer.finished() {
					fire_at_player();
				}

				continue;
			}
		}

		enemy.shock_timer.reset();

		if let EnemyAiState::Alert(target) = enemy.ai_state {
			let direction = (target - transform.translation.truncate()).normalize_or_zero();

			// If the enemy reached its destination
			if direction.length() == 0.0 {
				enemy.ai_state = EnemyAiState::Idle;
				continue;
			}

			let mut target = transform.translation + (direction * TILE_SIZE * movement.speed * time.delta_seconds()).extend(0.0);

			for wall_transform in wall_query.iter() {
				let collision = collide(
					target,
					Vec2::splat(TILE_SIZE),
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
}

fn fire_at_player() {
	println!("pew pew");
}
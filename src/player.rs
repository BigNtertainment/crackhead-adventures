use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use rand::prelude::*;

use crate::{TILE_SIZE, GameState};
use crate::HEIGHT;
use crate::WIDTH;
use crate::tilemap::{TileCollider, Tile};
use crate::unit::{Movement, Health};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerUi;

#[derive(Component)]
pub struct HealthBar;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.register_type::<Movement>()

			.add_system_set(
				SystemSet::on_enter(GameState::Game)
					.with_system(ui_setup)
			)

			.add_system_set(
				SystemSet::on_exit(GameState::Game)
					.with_system(drop_ui)
			)

			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(player_movement.label("player_movement"))
					.with_system(camera_follow.after("player_movement"))
					.with_system(damage_yourself)
					.with_system(update_ui)
					.with_system(player_aim)
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
	health: Health
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
			health: Health::new(100.0)
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

fn ui_setup(mut commands: Commands) {
	commands
		.spawn_bundle(NodeBundle {
			style: Style  {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				padding: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
			},
            color: Color::NONE.into(),
			..Default::default()
		})
		.insert(Name::new("UI"))
		.insert(PlayerUi)
		.with_children(|parent| {
			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Px(240.0), Val::Percent(100.0)),
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::FlexEnd,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.insert(Name::new("Bars"))
				.with_children(|parent| {
					parent
						.spawn_bundle(NodeBundle {
							style: Style {
								size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
								padding: UiRect::all(Val::Px(7.0)),
								..Default::default()
							},
							color: Color::rgb(0.0, 0.0, 0.0).into(),
							..Default::default()
						})
						.insert(Name::new("HealthBarContainer"))
						.with_children(|parent| {
							parent
								.spawn_bundle(NodeBundle {
									style: Style {
										size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
										..Default::default()
									},
									color: Color::rgb(0.95, 0.04, 0.07).into(),
									..Default::default()
								})
								.insert(Name::new("HealthBar"))
								.insert(HealthBar);
						});
				});
		});
}

fn drop_ui(mut commands: Commands, ui_query: Query<Entity, With<PlayerUi>>) {
	let ui = ui_query.single();
	commands.entity(ui).despawn_recursive();
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

fn update_ui(
	player_query: Query<&Health, With<Player>>,
	mut health_bar_query: Query<&mut Style, With<HealthBar>>
) {
	let player_health = player_query.single();
	let mut health_bar_style = health_bar_query.single_mut();

	health_bar_style.size.width = Val::Percent(player_health.get_health() / player_health.get_max_health() * 100.0);
}

fn damage_yourself(
	mut player_query: Query<&mut Health, With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	mut state: ResMut<State<GameState>>
) {
	let mut player_health = player_query.single_mut();

	if keyboard.just_pressed(KeyCode::Space) {
		if player_health.take_damage(rand::thread_rng().gen::<f32>() * 10.0 + 10.0) {
			state.set(GameState::GameOver).expect("Failed to change states");
		}
	}

}

fn player_aim(
	mut player_query: Query<&mut Transform, (With<Player>, Without<Camera2d>)>,
	window: Res<Windows>
) {
	let mut player_transform = player_query.single_mut();
	
	if let Some(target) = window.iter().next().unwrap().cursor_position(){
		let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);
		// gpu coords, from 0 to 1
		let ndc = (target / window_size) * 2.0 - Vec2::ONE;
		let angle = (Vec2::Y).angle_between(ndc);
		player_transform.rotation = Quat::from_rotation_z(angle);
	}
}
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use bevy_rapier2d::prelude::*;

use bevy_kira_audio::prelude::*;

use rand::prelude::*;

use crate::enemy::Enemy;
use crate::{TILE_SIZE, GameState};
use crate::HEIGHT;
use crate::WIDTH;
use crate::tilemap::{TileCollider, Tile};
use crate::unit::{Movement, Health, Inventory};

pub const WEAPON_RANGE: f32 = 400.0;
pub const WEAPON_COOLDOWN: f32 = 0.5;
pub const SMALL_POWERUP_DURATION: f32 = 5.0;
pub const BIG_POWERUP_DURATION: f32 = 5.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerUi;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct SmallPowerUpCounterNumber;

#[derive(Component)]
pub struct SmallPowerUpBar;

#[derive(Component)]
pub struct BigPowerUpCounterNumber;

#[derive(Component)]
pub struct BigPowerUpBar;

pub struct PaintFont(Handle<Font>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.register_type::<Movement>()

			.add_startup_system(load_shot_sound)
			.add_startup_system(load_font)

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
					.with_system(player_aim.label("player_aim").after("player_movement"))
					.with_system(player_shoot.after("player_aim"))
					.with_system(damage_yourself)
					.with_system(update_ui)
					.with_system(player_aim)
					.with_system(use_small_powerup)
					.with_system(use_big_powerup)
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
	shoot_cooldown: ShootCooldown,
	inventory: Inventory,
	small_power_up_cooldown: SmallPowerupCooldown,
	big_power_up_cooldown: BigPowerupCooldown,
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
			shoot_cooldown: ShootCooldown(Timer::new(Duration::from_secs_f32(WEAPON_COOLDOWN), false)),
			inventory: Inventory::new(20.0, -50.0),
			small_power_up_cooldown: SmallPowerupCooldown(Timer::new(Duration::from_secs_f32(SMALL_POWERUP_DURATION), false)),
			big_power_up_cooldown: BigPowerupCooldown(Timer::new(Duration::from_secs_f32(BIG_POWERUP_DURATION), false)),
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

fn ui_setup(mut commands: Commands, font: Res<PaintFont>) {
	let font = &font.0;

	commands
		.spawn_bundle(NodeBundle {
			style: Style  {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				padding: UiRect::all(Val::Px(20.0)),
				flex_direction: FlexDirection::ColumnReverse,
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
						size: Size::new(Val::Px(240.0), Val::Auto),
						margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(10.0)),
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
			
			parent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Px(240.0), Val::Percent(20.0)),
						flex_direction: FlexDirection::ColumnReverse,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.insert(Name::new("Inventory"))
				.with_children(|parent| {

					//SMALL POWER UP COUNTER
					parent
					.spawn_bundle(
						TextBundle::from_section(
							"Cocaine: ",
							TextStyle {
								font: font.clone(),
								font_size: 32.0,
								color: Color::PINK, // this needs changing
							},
						)
						.with_style(Style {
							size: Size::new(Val::Auto, Val::Auto),
							margin: UiRect::all(Val::Px(0.0)),
							..default()
						}),
					)
					.insert(Name::new("SmallPowerUpCounter"))
					.with_children(|parent| {
						parent
							.spawn_bundle(
								TextBundle::from_section(
									"0",
									TextStyle {
										font: font.clone(),
										font_size: 32.0,
										color: Color::PINK, // this needs changing
									},
								)
								.with_style(Style {
									size: Size::new(Val::Auto, Val::Auto),
									margin: UiRect::new(Val::Px(115.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
									..default()
								}),
							)
							.insert(Name::new("SmallPowerUpCounterNumber"))
							.insert(SmallPowerUpCounterNumber);
					});


					//SMALL POWER UP TIMER	
					parent
					.spawn_bundle(NodeBundle {
						style: Style {
							size: Size::new(Val::Px(150.0), Val::Px(10.0)),
							justify_content: JustifyContent::FlexStart,
							..Default::default()
						},
						color: Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.3 }.into(),
						..Default::default()
						
					})
					.insert(Name::new("SmallPowerUpBarContainer"))
					.with_children(|parent| {
						parent
						.spawn_bundle(NodeBundle {
							style: Style {
								size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
								..Default::default()
							},
							color: Color::WHITE.into(),
							..Default::default()
						})
						.insert(Name::new("SmallPowerUpBar"))
						.insert(SmallPowerUpBar);
					});
					

					//BIG POWER UP COUNTER
					parent
					.spawn_bundle(
						TextBundle::from_section(
							"Cocaine+1: ",
							TextStyle {
								font: font.clone(),
								font_size: 32.0,
								color: Color::PINK, // this needs changing
							},
						)
						.with_style(Style {
							size: Size::new(Val::Auto, Val::Auto),
							margin: UiRect::all(Val::Px(0.0)),
							..default()
						}),
					)
					.insert(Name::new("BigPowerUpCounter"))
					.with_children(|parent| {
						parent
							.spawn_bundle(
								TextBundle::from_section(
									"0",
									TextStyle {
										font: font.clone(),
										font_size: 32.0,
										color: Color::PINK, // this needs changing
									},
								)
								.with_style(Style {
									size: Size::new(Val::Auto, Val::Auto),
									margin: UiRect::new(Val::Px(155.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
									..default()
								}),
							)
							.insert(Name::new("BigPowerUpCounterNumber"))
							.insert(BigPowerUpCounterNumber);
					});

					//BIG POWER UP TIMER
					parent
					.spawn_bundle(NodeBundle {
						style: Style {
							size: Size::new(Val::Px(150.0), Val::Px(10.0)),
							justify_content: JustifyContent::FlexStart,
							..Default::default()
						},
						color:  Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.3 }.into(),
						..Default::default()
						
					})
					.insert(Name::new("BigPowerUpBarContainer"))
					.with_children(|parent| {
						parent
						.spawn_bundle(NodeBundle {
							style: Style {
								size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
								..Default::default()
							},
							color: Color::WHITE.into(),
							..Default::default()
						})
						.insert(Name::new("BigPowerUpBar"))
						.insert(BigPowerUpBar);
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
	player_query: Query<(&Health, &Inventory, &SmallPowerupCooldown, &BigPowerupCooldown), With<Player>>,
	mut health_bar_query: Query<&mut Style, (With<HealthBar>, Without<SmallPowerUpBar>, Without<BigPowerUpBar>)>,
	mut small_powerup_counter_query: Query<&mut Text, (With<SmallPowerUpCounterNumber>, Without<BigPowerUpCounterNumber>)>,
	mut small_powerup_bar_query: Query<&mut Style, (With<SmallPowerUpBar>, Without<BigPowerUpBar>, Without<HealthBar>)>,
	mut big_powerup_counter_query: Query<&mut Text, (With<BigPowerUpCounterNumber>, Without<SmallPowerUpCounterNumber>)>,
	mut big_powerup_bar_query: Query<&mut Style, (With<BigPowerUpBar>, Without<HealthBar>, Without<SmallPowerUpBar>)>,
) {
	let (player_health, inventory, small_cooldown, big_cooldown) = player_query.single();
	let mut health_bar_style = health_bar_query.single_mut();

	health_bar_style.size.width = Val::Percent(player_health.get_health() / player_health.get_max_health() * 100.0);

	let mut small_powerup_counter = small_powerup_counter_query.single_mut();
	small_powerup_counter.sections[0].value = inventory.get_small_powerup().to_string();

	let mut big_powerup_counter = big_powerup_counter_query.single_mut();
	big_powerup_counter.sections[0].value = inventory.get_big_powerup().to_string();

	let mut small_powerup_bar = small_powerup_bar_query.single_mut();
	small_powerup_bar.size.width = Val::Percent(small_cooldown.0.elapsed_secs() / SMALL_POWERUP_DURATION * 100.0);

	let mut big_powerup_bar = big_powerup_bar_query.single_mut();
	big_powerup_bar.size.width = Val::Percent(big_cooldown.0.elapsed_secs() / BIG_POWERUP_DURATION * 100.0);
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
	mut player_query: Query<&mut Transform, With<Player>>,
	window: Res<Windows>
) {
	let mut player_transform = player_query.single_mut();
	
	if let Some(target) = window.iter().next().unwrap().cursor_position(){
		let window_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

		let target = target - window_size / 2.0;

		let angle = (Vec2::Y).angle_between(target);
		player_transform.rotation = Quat::from_rotation_z(angle);
	}
}

fn load_shot_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
	let sound = asset_server.load("shot.wav");

	commands.insert_resource(ShotSound(sound));
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(
		PaintFont(asset_server.load("floyd.ttf"))
	);
}

struct ShotSound(Handle<AudioSource>);

#[derive(Component)]
pub struct ShootCooldown(Timer);

fn player_shoot(
	mut commands: Commands,
	mut player_query: Query<(&Transform, &mut ShootCooldown), With<Player>>,
	enemies_query: Query<Entity, With<Enemy>>,
	rapier_context: Res<RapierContext>,
	buttons: Res<Input<MouseButton>>,
	time: Res<Time>,
	window: Res<Windows>,
	audio: Res<Audio>,
	shot_sound: Res<ShotSound>
) {
	let (player_transform, mut cooldown) = player_query.single_mut();

	cooldown.0.tick(time.delta());

	if !cooldown.0.finished() {
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
		let filter = QueryFilter::default();

		if buttons.just_pressed(MouseButton::Left) {	
			if let Some((entity, _toi))  = rapier_context.cast_ray(
				ray_origin, ray_direction, max_time_of_impact, solid, filter
			) {
				for enemy in enemies_query.iter() {
					if entity.id() == enemy.id() {
						commands.entity(entity).despawn_recursive();
					}
				}
				
			}

			audio
				.play(shot_sound.0.clone())
				.with_volume(0.15);

			// Reset the cooldown timer
			cooldown.0.reset();
		}
	}
}

#[derive(Component)]
pub struct SmallPowerupCooldown(Timer);

fn use_small_powerup(
	mut player_query: Query<(&mut Health, &mut Inventory, &mut Movement, &mut SmallPowerupCooldown), With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (mut player_health, mut player_inventory, mut movement, mut cooldown) = player_query.single_mut();

	cooldown.0.tick(time.delta());

	if !cooldown.0.finished() {
		return;
	}

	if keyboard.just_pressed(KeyCode::E) {
		if player_inventory.subtract_small_powerup(1) {
			// add effects here!!
			player_health.heal(player_inventory.get_small_powerup_health());
			movement.speed *= 2.0;
			// Reset the cooldown timer
			cooldown.0.reset();
		}
	}

	if cooldown.0.just_finished() {
		// remove the effects here
		movement.speed /= 2.0;
	}
}

#[derive(Component)]
pub struct BigPowerupCooldown(Timer);

fn use_big_powerup(
	mut player_query: Query<(&mut Health, &mut Inventory, &mut Movement, &mut BigPowerupCooldown), With<Player>>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let (mut player_health, mut player_inventory, mut movement, mut cooldown) = player_query.single_mut();

	cooldown.0.tick(time.delta());

	if !cooldown.0.finished() {
		return;
	}

	if keyboard.just_pressed(KeyCode::R) {
		if player_inventory.subtract_big_powerup(1) {
			// add effects here!!
			player_health.set_health(5f32);
			movement.speed *= 2.0;
			// Reset the cooldown timer
			cooldown.0.reset();
		}
	}

	if cooldown.0.just_finished() {
		// remove the effects here
		movement.speed /= 2.0;
	}
}
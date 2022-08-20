use std::fs::File;
use std::io::{prelude::*, BufReader};
use rand::seq::SliceRandom;

use bevy::prelude::*;

use crate::{TILE_SIZE, GameState};
use crate::enemy::EnemyBundle;
use crate::player::PlayerBundle;

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(load_wall_textures)
			.add_system_set(
				SystemSet::on_enter(GameState::Game)
					.with_system(load_level.label("load_level"))
					.with_system(texture_walls.after("load_level"))
			)
			.add_system_set(SystemSet::on_exit(GameState::Game).with_system(drop_level));
	}
}

pub trait Tile {
	fn at(position: Vec2) -> Self;
}

// Tiles
#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	collider: TileCollider,
	wall: Wall
}

impl Default for WallBundle {
	fn default() -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				// sprite: Sprite {
				// 	color: Color::rgb(0.75, 0.25, 0.25),
				// 	custom_size: Some(Vec2::splat(TILE_SIZE)),
				// 	..Default::default()
				// },
				..Default::default()
			},
			collider: TileCollider,
			wall: Wall
		}
	}
}

impl Tile for WallBundle {
	fn at(position: Vec2) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				// sprite: Sprite {
				// 	color: Color::rgb(66.0 / 255.0, 135.0 / 255.0, 245.0 / 255.0),
				// 	custom_size: Some(Vec2::splat(TILE_SIZE)),
				// 	..Default::default()
				// },
				transform: Transform::from_xyz(position.x, position.y, 0.0),
				..Default::default()
			},
			..Default::default()
		}
	}
}

struct WallTextures(Vec<Handle<Image>>);

fn load_wall_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
	let mut textures = Vec::new();

	textures.push(asset_server.load("brick_1.png"));
	textures.push(asset_server.load("brick_2.png"));

	commands.insert_resource(WallTextures(textures));
}

fn texture_walls(mut walls: Query<&mut Handle<Image>, With<Wall>>, textures: Res<WallTextures>) {
	println!("a");

	for mut wall_texture in walls.iter_mut() {
		println!("b");
		wall_texture.id = textures.0.choose(&mut rand::thread_rng()).expect("The wall textures vector is empty!").id;
	}
}

fn load_level(mut commands: Commands) {
	let file = File::open("assets/level.txt").expect("Level file (level.txt) not found!");

	let mut tiles = Vec::new();

	for (y, line) in BufReader::new(file).lines().enumerate() {
		if let Ok(line) = line {
			for (x, char) in line.chars().enumerate() {
				let tile = spawn_tile(
					&mut commands,
					char,
					Vec2::new(x as f32, -(y as f32))
				);

				match tile {
					Ok(tile) => {
						if let Some(tile) = tile {
							tiles.push(tile);
						}
					},
					Err(err) => {
						match err {
							TileSpawnError::UnknownChar(char) => {
								panic!("Unknown char in the level file on position ({}, {}): {}", x, y, char);
							}
						}
					}
				};
			}
		}
	}

	commands
		.spawn()
		.insert(Name::new("Tilemap"))
		.insert(Visibility::default())
		.insert(ComputedVisibility::default())
		.insert(Transform::default())
		.insert(GlobalTransform::default())
		.insert(Tilemap)
		.push_children(&tiles);

	println!("c");
}

fn drop_level(mut commands: Commands, tilemap: Query<Entity, With<Tilemap>>) {
	let tilemap = tilemap.single();
	commands.entity(tilemap).despawn_recursive();
}

enum TileSpawnError {
	UnknownChar(char)
}

fn spawn_tile(commands: &mut Commands, tile_char: char, position_on_tilemap: Vec2) -> Result<Option<Entity>, TileSpawnError> {
	let position = position_on_tilemap * TILE_SIZE;

	return match tile_char {
		'#' => {
			Ok(Some(commands.spawn_bundle(WallBundle::at(position)).id()))
		},
		'O' => {
			Ok(Some(commands.spawn_bundle(PlayerBundle::at(position)).id()))
		},
		'E' => {
			Ok(Some(commands.spawn_bundle(EnemyBundle::at(position)).id()))
		},
		' ' => {
			Ok(None)
		},
		unknown => {
			Err(TileSpawnError::UnknownChar(unknown))
		}
	};
}
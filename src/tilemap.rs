use std::path::PathBuf;
use bevy::utils::HashMap;

use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use tiled::{Loader, TileLayer, LayerType, Chunk};

use crate::{TILE_SIZE, GameState};
use crate::enemy::EnemyBundle;
use crate::player::PlayerBundle;
use crate::enemy_nav_mesh::EnemyNavMesh;

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(EnemyNavMesh::new())
			.insert_resource(TexturesMemo {
				memoized: HashMap::new()
			})

			.add_system_set(
				SystemSet::on_enter(GameState::Game)
					.with_system(load_level.label("load_level"))
			)

			.add_system_set(
				SystemSet::on_exit(GameState::Game)
					.with_system(drop_level)
					.with_system(|mut nav_mesh: ResMut<EnemyNavMesh>| { nav_mesh.clear(); })
			);
	}
}

pub trait Tile {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self;
}

// Tiles
#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	collider: TileCollider,
	rapier_collider: Collider,
	wall: Wall,
	name: Name
}

impl Default for WallBundle {
	fn default() -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				..Default::default()
			},
			collider: TileCollider,
			wall: Wall,
			rapier_collider: Collider::cuboid(TILE_SIZE/2.0, TILE_SIZE/2.0),
			name: Name::new("Wall")
		}
	}
}

impl Tile for WallBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				transform: Transform::from_xyz(position.x, position.y, 0.0),
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
struct FloorBundle {
	#[bundle]
	sprite_bundle: SpriteBundle,
	name: Name,
}

impl Default for FloorBundle {
	fn default() -> Self {
		Self {
			sprite_bundle: Default::default(),
			name: Name::new("Floor")
		}
	}
}

impl Tile for FloorBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				transform: Transform::from_xyz(position.x, position.y, 0.0),
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

struct TexturesMemo {
	memoized: HashMap<PathBuf, Handle<Image>>,
}

impl TexturesMemo {
	pub fn get(&mut self, path: &PathBuf, asset_server: &Res<AssetServer>) -> Handle<Image> {
		if let Some(handle) = self.memoized.get(path) {
			handle.clone()
		} else {
			let handle = asset_server.load(path.to_str().unwrap());
			self.memoized.insert(path.clone(), handle.clone());
			handle
		}
	}
}

fn load_level(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<TexturesMemo>,
	mut nav_mesh: ResMut<EnemyNavMesh>
) {
	let mut loader = Loader::new();
	let map = loader.load_tmx_map("assets/level/level.tmx").unwrap();
    let tileset = loader.load_tsx_tileset("assets/level/tileset.tsx").unwrap();
	let layers = map.layers();

	let mut entities = Vec::new();

	for (layer_num, layer) in layers.enumerate() {
		match layer.layer_type() {
			LayerType::TileLayer(layer) => {
				match layer {
					TileLayer::Finite(_) => {
						todo!("Implement this if there's time left.");
					},
					TileLayer::Infinite(layer) => {
						for (chunk_pos, chunk) in layer.chunks() {
							for x in 0..Chunk::WIDTH as i32 {
								for y in 0..Chunk::HEIGHT as i32 {
									if let Some(tile) = chunk.get_tile_data(x, y) {
										let tile_pos = Vec2::new(
											(chunk_pos.0 * Chunk::WIDTH as i32 + x) as f32,
											(chunk_pos.1 * Chunk::HEIGHT as i32 + y) as f32,
										) * TILE_SIZE;

										let mut register_nav_mesh = || {
											nav_mesh.insert_square(
												tile_pos + Vec2::new(map.tile_width as f32, map.tile_height as f32),
												tile_pos + Vec2::new(-(map.tile_width as f32), map.tile_height as f32),
												tile_pos + Vec2::new(-(map.tile_width as f32), -(map.tile_height as f32)),
												tile_pos + Vec2::new(map.tile_width as f32, -(map.tile_height as f32)),
											);
										};

										let (flip_x, flip_y) = (
											tile.flip_h || tile.flip_d,
											tile.flip_v || tile.flip_d,
										);

										let tile = tileset.get_tile(tile.id()).expect("what");

										let image_source =
											tile.image.as_ref().unwrap()
											.source.strip_prefix("assets/level\\..")
											.expect("what").to_path_buf();

										entities.push(match layer_num {
											0 => {
												// Floor layer
												register_nav_mesh();

												commands
													.spawn_bundle(FloorBundle::spawn(
														tile_pos, 
														textures.get(
															&image_source,
															&asset_server
														),
														flip_x,
														flip_y,
													))
											},
											1 => {
												// Wall layer
												commands
													.spawn_bundle(WallBundle::spawn(
														tile_pos, 
														textures.get(
															&image_source,
															&asset_server
														),
														flip_x,
														flip_y,
													))
											},
											2 => {
												// Player layer
												commands
													.spawn_bundle(PlayerBundle::spawn(
														tile_pos, 
														textures.get(
															&image_source,
															&asset_server
														),
														flip_x,
														flip_y,
													))
											},
											3 => {
												// Enemy layer
												commands
												.spawn_bundle(EnemyBundle::spawn(
													tile_pos, 
													textures.get(
														&image_source,
														&asset_server
													),
													flip_x,
													flip_y,
												))
											},
											_ => {
												panic!("Too much layers in the level file");
											}
										}.id());
									}
								}
							}
						}
					}
				}
			},
			_ => panic!("The level has a different kind of layer than a tile layer!")
		}
	}

	nav_mesh.bake();

	commands
		.spawn()
		.insert(Name::new("Tilemap"))
		.insert(Visibility::default())
		.insert(ComputedVisibility::default())
		.insert(Transform::default())
		.insert(GlobalTransform::default())
		.insert(Tilemap)
		.push_children(&entities);
}

fn drop_level(mut commands: Commands, tilemap: Query<Entity, With<Tilemap>>) {
	let tilemap = tilemap.single();
	commands.entity(tilemap).despawn_recursive();
}

fn draw_nav_mesh(nav_mesh: ResMut<EnemyNavMesh>, mut lines: ResMut<DebugLines>) {
	nav_mesh.draw(&mut lines);
}

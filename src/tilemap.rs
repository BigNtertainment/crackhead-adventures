use bevy::utils::HashMap;
use std::io::ErrorKind;
use std::path::{PathBuf, Path};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use tiled::{Chunk, LayerType, Loader, TileLayer, Tileset, Map, ResourceReader, ResourceCache, DefaultResourceCache};

use crate::cocaine::CocaineBundle;
use crate::enemy::EnemyBundle;
use crate::enemy_nav_mesh::EnemyNavMesh;
use crate::player::PlayerBundle;
use crate::win::{WinBundle, WinMaterial};
use crate::{GameState, TILE_SIZE};

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(EnemyNavMesh::new())
			.insert_resource(TexturesMemo {
				memoized: HashMap::new(),
			})
			.add_system_set(
				SystemSet::on_enter(GameState::Game).with_system(load_level.label("load_level")),
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Game)
					.with_system(drop_level)
					.with_system(|mut nav_mesh: ResMut<EnemyNavMesh>| {
						nav_mesh.clear();
					}),
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
	name: Name,
}

impl Default for WallBundle {
	fn default() -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				..Default::default()
			},
			collider: TileCollider,
			wall: Wall,
			rapier_collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
			name: Name::new("Wall"),
		}
	}
}

impl Tile for WallBundle {
	fn spawn(position: Vec2, texture: Handle<Image>, flip_x: bool, flip_y: bool) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				transform: Transform::from_xyz(position.x, position.y, 10.0),
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
			name: Name::new("Floor"),
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

pub struct TexturesMemo {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct WasmResourceReader;

impl ResourceReader for WasmResourceReader {
	type Resource = &'static [u8];
	type Error = std::io::Error;

	fn read_from(&mut self, path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
		match path.to_str().expect("Given path is not a valid unicode string") {
			"level.tmx" => {
				Ok(&include_bytes!("../assets/level/level.tmx")[..])
			},
			"tileset.tsx" => {
				Ok(&include_bytes!("../assets/level/tileset.tsx")[..])
			},
			other => {
				Err(std::io::Error::new(ErrorKind::Unsupported, format!("\"{}\" is not a valid option for the WasmReader", other)))
			}
		}
	}
}

trait SimpleNew {
	fn new() -> Self;
}

fn load_tilemap() -> (Map, Tileset) {
	let mut loader = Loader::<DefaultResourceCache, WasmResourceReader>::new();

	(
		loader.load_tmx_map("level.tmx").unwrap(),
		loader.load_tsx_tileset("tileset.tsx").unwrap(),
	)
}

fn load_level(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<TexturesMemo>,
	mut nav_mesh: ResMut<EnemyNavMesh>,
	mut win_materials: ResMut<Assets<WinMaterial>>,
) {
	let (map, tileset) = load_tilemap();

	let layers = map.layers();

	let mut entities = Vec::new();

	let is_wall_at = |x: i32, y: i32| {
		let wall_layer = map
			.get_layer(1)
			.expect("There is no wall layer in the map file!");

		match wall_layer.layer_type() {
			LayerType::Tiles(wall_layer) => match wall_layer {
				TileLayer::Finite(_) => todo!("Maybe do this someday"),
				TileLayer::Infinite(wall_layer) => {
					return wall_layer.get_tile(x, y).is_some();
				}
			},
			_ => {
				panic!("Wall layer is of an incorrect type (not a tile layer)!");
			}
		}
	};

	let mut register_nav_mesh = |x, y| {
		let top = (y as f32 + if !is_wall_at(x, y - 1) { -0.5 } else { 0.0 }) * TILE_SIZE;
		let bottom = (y as f32 + if !is_wall_at(x, y + 1) { 0.5 } else { 0.0 }) * TILE_SIZE;
		let left = (x as f32 + if !is_wall_at(x - 1, y) { -0.5 } else { 0.0 }) * TILE_SIZE;
		let right = (x as f32 + if !is_wall_at(x + 1, y) { 0.5 } else { 0.0 }) * TILE_SIZE;

		nav_mesh.insert_rect(
			Vec2::new(left, -top),
			Vec2::new(right, -top),
			Vec2::new(right, -bottom),
			Vec2::new(left, -bottom),
		);
	};

	for (layer_num, layer) in layers.enumerate() {
		match layer.layer_type() {
			LayerType::Tiles(layer) => {
				match layer {
					TileLayer::Finite(_) => {
						todo!("Implement this if there's time left.");
					}
					TileLayer::Infinite(layer) => {
						for (chunk_pos, chunk) in layer.chunks() {
							for x in 0..Chunk::WIDTH as i32 {
								for y in 0..Chunk::HEIGHT as i32 {
									if let Some(tile) = chunk.get_tile_data(x, y) {
										let tile_pos = Vec2::new(
											(chunk_pos.0 * Chunk::WIDTH as i32 + x) as f32,
											-(chunk_pos.1 * Chunk::HEIGHT as i32 + y) as f32,
										) * TILE_SIZE;

										let (flip_x, flip_y) = (
											tile.flip_h || tile.flip_d,
											tile.flip_v || tile.flip_d,
										);

										if let Some(tile) = tileset.get_tile(tile.id()) {
											// println!("{}", tile
											// .image
											// .as_ref()
											// .unwrap()
											// .source.display());

											let image_source = tile
												.image
												.as_ref()
												.unwrap()
												.source
												.strip_prefix("..")
												.expect("what")
												.to_path_buf();

											let image_source = Path::new("./").join(image_source);

											entities.push(
												match layer_num {
													0 => {
														// Floor layer
														register_nav_mesh(
															chunk_pos.0 * Chunk::WIDTH as i32 + x,
															chunk_pos.1 * Chunk::HEIGHT as i32 + y,
														);

														commands.spawn_bundle(FloorBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).id()
													}
													1 => {
														// Wall layer
														commands.spawn_bundle(WallBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).id()
													}
													2 => {
														// Player layer
														commands.spawn_bundle(PlayerBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).id()
													}
													3 => {
														// Enemy layer
														commands.spawn_bundle(EnemyBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).id()
													}
													4 => {
														// Cocaine layer
														commands.spawn_bundle(CocaineBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).id()
													}
													5 => {
														// Details layer
														commands.spawn_bundle(SpriteBundle {
															transform: Transform::from_translation(
																tile_pos.extend(20.0),
															),
															sprite: Sprite {
																flip_x,
																flip_y,
																..Default::default()
															},
															texture: textures
																.get(&image_source, &asset_server),
															..Default::default()
														}).id()
													}
													6 => {
														// Win layer
														let material =
															win_materials.add(WinMaterial {
																source_image: textures.get(
																	&image_source,
																	&asset_server,
																),
																time: 0,
															});

														commands.spawn_bundle(WinBundle::spawn(
															tile_pos,
															textures
																.get(&image_source, &asset_server),
															flip_x,
															flip_y,
														)).insert(material).id()
													}
													_ => {
														panic!("Too much layers in the level file");
													}
												},
											);
										}
									}
								}
							}
						}
					}
				}
			}
			_ => panic!("The level has a different kind of layer than a tile layer!"),
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

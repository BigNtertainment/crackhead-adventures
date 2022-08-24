use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::render_resource::{AsBindGroup, ShaderRef},
	sprite::{Material2d, Material2dPlugin},
};

use crate::{post_processing::{
	update_post_processing_effects, CameraRenderImage, DefaultMaterial, PostProcessingLayer, Screen,
}, GameState};

pub struct PlayerPostProcessingPlugin;

impl Plugin for PlayerPostProcessingPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(Material2dPlugin::<SmallPowerupMaterial>::default())
			.add_plugin(Material2dPlugin::<BigPowerupMaterial>::default())
			.add_system_set(SystemSet::on_exit(GameState::Game).with_system(clean_post_processing_effect));
	}
}

pub fn clean_post_processing_effect(
	mut commands: Commands,
	screen: Query<Entity, With<Screen>>,
	mut meshes: ResMut<Assets<Mesh>>,
	post_processing_pass_layer: Res<PostProcessingLayer>,
	source_image: Res<CameraRenderImage>,
	mut materials: ResMut<Assets<DefaultMaterial>>,
) {
	clean_post_processing(&mut commands, &screen.single(), &mut meshes, &post_processing_pass_layer, &source_image, &mut materials)
}

pub fn clean_post_processing(
	commands: &mut Commands,
	screen: &Entity,
	meshes: &mut ResMut<Assets<Mesh>>,
	post_processing_pass_layer: &Res<PostProcessingLayer>,
	source_image: &Res<CameraRenderImage>,
	default_materials: &mut ResMut<Assets<DefaultMaterial>>,
) {
	let material = DefaultMaterial {
		source_image: source_image.0.clone(),
	};

	let material_handle = default_materials.add(material);

	update_post_processing_effects(
		commands,
		screen,
		material_handle,
		meshes,
		&post_processing_pass_layer,
	);
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "2cf6b1b2-23f2-11ed-861d-0242ac120002"]
pub struct SmallPowerupMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
}

impl Material2d for SmallPowerupMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material_chromatic_aberration.wgsl".into()
	}
}


#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "2cf6b4dc-23f2-11ed-861d-0242ac120002"]
pub struct BigPowerupMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
}

impl Material2d for BigPowerupMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material_chromatic_aberration.wgsl".into()
	}
}

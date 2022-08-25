use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::{
		camera::RenderTarget,
		render_resource::{
			AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
			TextureUsages,
		},
		texture::BevyDefault,
		view::RenderLayers,
	},
	sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::{HEIGHT, WIDTH};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(Material2dPlugin::<DefaultMaterial>::default())
			.add_startup_system(setup);
	}
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Screen;

#[derive(Deref, DerefMut)]
pub struct PostProcessingLayer(pub RenderLayers);

#[derive(Deref, DerefMut)]
pub struct CameraRenderImage(pub Handle<Image>);

#[derive(Deref, DerefMut)]
pub struct ScreenRes(pub Entity);

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut post_processing_materials: ResMut<Assets<DefaultMaterial>>,
	mut images: ResMut<Assets<Image>>,
) {
	let size = Extent3d {
		width: WIDTH as u32,
		height: HEIGHT as u32,
		..Default::default()
	};

	// This is the texture that will be rendered to.
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::bevy_default(),
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
		},
		..default()
	};

	// Fill image.data with zeroes
	image.resize(size);

	let image_handle = images.add(image);

	// Add main camera rendering to the image
	commands
		.spawn_bundle(Camera2dBundle {
			camera: Camera {
				target: RenderTarget::Image(image_handle.clone()),
				priority: 0,
				..Default::default()
			},
			..Default::default()
		})
		.insert(MainCamera)
		.insert(Name::new("MainCamera"));

	// This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
	let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

	let post_processing_pass_layer_resource = PostProcessingLayer(post_processing_pass_layer);

	// This material has the texture that has been rendered.
	let material_handle = post_processing_materials.add(DefaultMaterial {
		source_image: image_handle.clone(),
	});

	set_post_processing_effects(
		&mut commands,
		material_handle,
		&mut meshes,
		&post_processing_pass_layer_resource,
	);

	commands.insert_resource(post_processing_pass_layer_resource);
	commands.insert_resource(CameraRenderImage(image_handle));

	// The post-processing pass camera.
	commands
		.spawn_bundle(Camera2dBundle {
			camera: Camera {
				// renders after the first main camera which has default value: 0.
				priority: 1,
				..default()
			},
			..Camera2dBundle::default()
		})
		.insert(post_processing_pass_layer)
		.insert(Name::new("SecondaryCamera"));
}

fn set_post_processing_effects<M: Material2d>(
	commands: &mut Commands,
	material: Handle<M>,
	meshes: &mut ResMut<Assets<Mesh>>,
	post_processing_pass_layer: &PostProcessingLayer,
) {
	let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
		WIDTH as f32,
		HEIGHT as f32,
	))));

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	let screen = commands
		.spawn_bundle(MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		})
		.insert(post_processing_pass_layer.0)
		.insert(Name::new("Screen"))
		.insert(Screen).id();

	commands.insert_resource(ScreenRes(screen));
}

pub fn update_post_processing_effects<M: Material2d>(
	commands: &mut Commands,
	screen: &Entity,
	material: Handle<M>,
	meshes: &mut ResMut<Assets<Mesh>>,
	post_processing_pass_layer: &PostProcessingLayer,
) {
	commands.entity(*screen).despawn_recursive();

	set_post_processing_effects(commands, material, meshes, post_processing_pass_layer);
}

/// Default post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct DefaultMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
}

impl Material2d for DefaultMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/default.wgsl".into()
	}
}

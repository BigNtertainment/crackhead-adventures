use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_resource::{AsBindGroup, ShaderRef, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
		view::RenderLayers, texture::BevyDefault, camera::RenderTarget,
	},
	sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
			.add_startup_system(setup);
	}
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut images: ResMut<Assets<Image>>,
	mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
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

	let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
		size.width as f32,
		size.height as f32,
	))));

	// This material has the texture that has been rendered.
	let material_handle = post_processing_materials.add(PostProcessingMaterial {
		source_image: image_handle,
	});

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	commands
		.spawn_bundle(MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material_handle,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		})
		.insert(post_processing_pass_layer)
		.insert(Name::new("Screen"));

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


/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct PostProcessingMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material_chromatic_aberration.wgsl".into()
	}
}

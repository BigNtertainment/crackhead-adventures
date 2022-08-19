use bevy::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 50.0;

mod player;
mod debug;

use player::PlayerPlugin;
use debug::DebugPlugin;

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            ..Default::default()
        });
}

fn main() {
    App::new()
        // Resources
        .insert_resource(WindowDescriptor {
            title: "Angel Dust".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..Default::default()
        })

        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)

        // Systems
        .add_startup_system(camera_setup)

        .run();
}

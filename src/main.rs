use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 50.0;

mod player;
mod debug;
mod tilemap;

use player::PlayerPlugin;
use debug::DebugPlugin;
use tilemap::TileMapPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Game
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        // States
        .add_state(GameState::Game)

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)

        // Systems
        .add_startup_system(camera_setup)

        .run();
}

#![deny(unused_must_use)]

use bevy::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 50.0;

mod unit;
mod player;
mod enemy;
mod debug;
mod tilemap;
mod game_over;

use player::PlayerPlugin;
use debug::DebugPlugin;
use tilemap::TileMapPlugin;
use game_over::GameOverPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Game,
    GameOver
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
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(DebugPlugin)

        // Systems
        .add_startup_system(camera_setup)

        .run();
}

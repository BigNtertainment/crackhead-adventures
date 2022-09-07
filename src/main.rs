#![deny(unused_must_use)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_kira_audio::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 50.0;

mod unit;
mod player;
mod enemy;
mod cocaine;
mod bullet;
mod debug;
mod tilemap;
mod win;
mod fonts;
mod button;
mod post_processing;
mod main_menu;
mod game_over;
mod crosshair;
mod enemy_nav_mesh;
mod audio;
mod music;

use bullet::BulletPlugin;
use button::ButtonPlugin;
use crosshair::CrosshairPlugin;
use enemy::EnemyPlugin;
use fonts::FontPlugin;
use main_menu::MainMenuPlugin;
use music::MusicPlugin;
use player::PlayerPlugin;
use debug::DebugPlugin;
use post_processing::PostProcessingPlugin;
use tilemap::TileMapPlugin;
use game_over::GameOverPlugin;
use audio::AudioLoadPlugin;
use win::WinPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    Game,
    GameOver,
    Win,
}

fn main() {
    App::new()
        // States
        .add_state(GameState::MainMenu)

        // Resources
        .insert_resource(WindowDescriptor {
            title: "Crackhead Adventures".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..Default::default()
        })

        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(AudioPlugin)
        .add_plugin(MusicPlugin)
        .add_plugin(PostProcessingPlugin)
        .add_plugin(AudioLoadPlugin)
        .add_plugin(FontPlugin)
        .add_plugin(ButtonPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(WinPlugin)
        .add_plugin(CrosshairPlugin)
        .add_plugin(DebugPlugin)

        .run();
}

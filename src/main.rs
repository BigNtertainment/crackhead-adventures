#![deny(unused_must_use)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_kira_audio::prelude::*;

// we dont need those things in wasm build since they are for setting window icon
#[cfg(not(target_arch="wasm32"))]
use bevy::window::WindowId;
#[cfg(not(target_arch="wasm32"))]
use bevy::winit::WinitWindows;
use time::TimePlugin;
#[cfg(not(target_arch="wasm32"))]
use winit::window::Icon;


pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 50.0;

mod unit;
mod player;
mod enemy;
mod cocaine;
mod bullet;
mod time;
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
mod audio_player;
mod music;
mod stats;
mod settings;

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
use settings::SettingsPlugin;
use tilemap::TileMapPlugin;
use game_over::GameOverPlugin;
use audio::AudioLoadPlugin;
use win::WinPlugin;
use stats::StatsPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    Settings,
    Game,
    GameOver,
    Win,
    Stats,
}

#[cfg(not(target_arch="wasm32"))]
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("./assets/img/enemy_ded.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

// We cant set window icon on website so we have to do this thing
#[cfg(target_arch="wasm32")]
fn set_window_icon(){}

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

        // Setting window icon
        .add_startup_system(set_window_icon)

        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(TimePlugin)
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
        .add_plugin(StatsPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(DebugLinesPlugin::default())

        .run();
}

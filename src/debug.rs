use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(DebugLinesPlugin::default());
        }
    }
}

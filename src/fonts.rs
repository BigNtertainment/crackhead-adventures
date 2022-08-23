use bevy::prelude::*;

pub struct PaintFont(pub Handle<Font>);
pub struct RobotoFont(pub Handle<Font>);

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_fonts);
    }
}

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PaintFont(asset_server.load("fonts/floyd.ttf")));
    commands.insert_resource(RobotoFont(asset_server.load("fonts/roboto.ttf")));
}

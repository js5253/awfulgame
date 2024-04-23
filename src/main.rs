use bevy::prelude::*;
mod asset_loader;
mod player;
mod base;
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::ORANGE))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(asset_loader::AssetLoaderPlugin)
    .run();
}
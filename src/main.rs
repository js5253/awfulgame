mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;
mod schedule;
mod score;
use despawn::DespawnPlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use asteroids::AsteroidPlugin;
use score::{ScoreChange, ScorePlugin};
use spaceship::SpaceshipPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75
    })
    .add_event::<ScoreChange>()
    .add_plugins(ScorePlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DespawnPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(CollisionDetectionPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(DefaultPlugins
    //     .set(a   
    //     WindowPlugin {
    //         primary_window: Some(Window {
    //             present_mode: bevy::window::PresentMode::Immediate,
    //             ..default()
    //         }),
    //         ..default()
    //     }
    // )
)
    .run();  
}





// #[derive(Component, Debug)] 
// struct Position {
//     x: f32,
//     y: f32
// }




#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod ui;
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
mod health;
use despawn::DespawnPlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use health::HealthChange;
use movement::MovementPlugin;
use asteroids::AsteroidPlugin;
use score::{ScoreChange, ScorePlugin};
use spaceship::SpaceshipPlugin;
use bevy::prelude::*;
use ui::UiPlugin;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.87, 0.0, 0.15)))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75
    })
    .add_event::<ScoreChange>()
    .add_event::<HealthChange>()
    .add_plugins(UiPlugin)
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




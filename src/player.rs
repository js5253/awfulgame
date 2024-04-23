use bevy::{prelude::*, scene};

use crate::{asset_loader::SceneAssets, base::{Acceleration, MovingObjectBundle, Velocity}};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: scene_assets.beat.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
    }, Player));
}
use bevy::prelude::*;

use crate::spaceship::Spaceship;

const CAMERA_DISTANCE: f32 = 80.;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, move_camera_to_player);
    }
}

fn move_camera_to_player(mut commands: Commands, player: Query<&Transform, With<Spaceship>>, mut camera: Query<(&mut Camera3d, &mut Transform), Without<Spaceship>>) {
    let player = player.get_single().unwrap();

        for (camera, mut camera_transform) in camera.iter_mut() {
            camera_transform.translation = player.translation;
        }
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., CAMERA_DISTANCE, 0.).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}, score::ScoreChange};


const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const SPACESHIP_SPEED: f32 = 25.;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 20.;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const SPACESHIP_RADIUS: f32 = 1.;
const MISSILE_RADIUS: f32 = 0.65;
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObjectBundle {
        acceleration: Acceleration::new(Vec3::ZERO),
        velocity: Velocity::new(Vec3::ZERO),
        collider: Collider::new(SPACESHIP_RADIUS),
        model: SceneBundle {
            scene: scene_assets.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    }, Spaceship)
);
}
#[derive(Component)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
        .add_systems(Update, (spaceship_movement_controls, spaceship_weapon_controls));
    }
}
#[derive(Component, Debug)]
pub struct Spaceship;
fn spaceship_movement_controls(mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>){
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.;
    let mut roll = 0.;
    let mut movement = 0.;

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Spaceship>>, keyboard_input: Res<ButtonInput<KeyCode>>, scene_assets: Res<SceneAssets>, mut score_change_writer: EventWriter<ScoreChange>) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        // score_change_writer.send(ScoreChange::Increment);
        commands.spawn((MovingObjectBundle {
            velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILE_RADIUS),
            model: SceneBundle {
                scene: scene_assets.missiles.clone(),
                transform: Transform::from_translation(transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR),
                ..default()
            },
            
        }, SpaceshipMissile)
    );
    }

}
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::{asset_loader::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}, player_enemy::PlayerEnemy, score::ScoreChange};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 1.75;
// add seeking to this
#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer
}
pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating)
        }).add_systems(Update, (spawn_asteroid, rotate_asteroid, handle_asteroid_collisions));
    }
}


fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    
    let translation = Vec3::new(rng.gen_range(SPAWN_RANGE_X), 0., rng.gen_range(SPAWN_RANGE_Z));
    let mut random_unit_vector = || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0));
    let velocity = random_unit_vector();
    let acceleration = random_unit_vector();

    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(velocity),
        collider: Collider::new(RADIUS),
        acceleration: Acceleration::new(acceleration),
        model: SceneBundle {
            scene: scene_assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        }
    }, PlayerEnemy {
        max_speed: 300,
        damage: 20
    }, Asteroid));
}
fn rotate_asteroid(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds())
    }
}
fn handle_asteroid_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Asteroid>>,     mut score_change_writer: EventWriter<ScoreChange>) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entries.iter() {
            if query.get(collided_entity).is_ok() {
                continue
            }
            score_change_writer.send(ScoreChange::Increment);
            commands.entity(entity).despawn_recursive();
        }
    }
}
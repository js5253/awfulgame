use bevy::prelude::*;

use crate::movement::MovingObjectBundle;

#[derive(Component, Debug)]
pub struct PlayerEnemy {
    pub damage: i32,
    pub max_speed: i32,
}

pub struct PlayerEnemyPlugin;
impl Plugin for PlayerEnemyPlugin {
 fn build(&self, app: &mut App) {
    //  app.add_systems(Update, route_entity_to_player);
 }   
}
fn route_entity_to_player(commands: Commands, query: Query<(Entity, &mut Transform), With<PlayerEnemy>>) {
    for (entity, transform) in query.iter() {
        println!("Enemy {:?}, {:?}", entity, transform);
    }
}

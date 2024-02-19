use bevy::prelude::*;

use crate::{schedule::InGameSet, score::Score};


fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, transform);
    }
}
fn print_score(score: Res<Score>) {
    println!("{}", score.value);
}
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(Update, print_position.after(InGameSet::EntityUpdates));
        .add_systems(Update, print_score);
    }
}
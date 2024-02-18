use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, Clone, PartialEq, Eq)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities
}

pub struct SchedulePlugin;
impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            InGameSet::DespawnEntities,
            InGameSet::UserInput,
            InGameSet::EntityUpdates,
            InGameSet::DespawnEntities
        )
    .chain(),)
    .add_systems(Update, apply_deferred.after(InGameSet::DespawnEntities).before(InGameSet::UserInput));
    }
}
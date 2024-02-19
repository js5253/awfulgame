use bevy::prelude::*;

struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
#[derive(PartialEq, Eq, Hash, Event, Debug)]
pub enum HealthChange {
    Increment,
    Decrement
}

#[derive(Component)]
pub struct Health {
    pub value: i32
}


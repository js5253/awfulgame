// COMMON STUFF (INCLUDES MOVING OBJECT BUNDLE, ETC)
use bevy::prelude::*;
use derive_new::new;
#[derive(Component, Debug, Default, new)]
pub struct Velocity {
    pub value: Vec3
}
#[derive(Component, Default, new)]
pub struct Acceleration {
    pub value: Vec3
}
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle
}
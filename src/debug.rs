use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::{schedule::InGameSet, score::Score, spaceship::Spaceship};

#[derive(Component)]
struct DebugFpsText;

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, transform);
    }
}
fn print_score(score: Res<Score>) {
}
fn debug_camera(player_query: Query<&Transform, With<Spaceship>>, camera_query: Query<&Transform, Without<Spaceship>>) {
    let Ok(player_camera) = player_query.get_single() else {
        return
    };
    let Ok(world_camera) = camera_query.get_single() else {
        return
    };

    println!("Player: {:?}, Camera: {:?}", player_camera.translation, world_camera.translation);
}
fn debug_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<DebugFpsText>>) {
    for mut diagnostics_text in query.iter_mut() {
        let diagnostics = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed()).unwrap();
        diagnostics_text.sections[0].value = diagnostics.to_string();
        
    }
}
fn setup_fps_text(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text::from_section("FPS", TextStyle::default()),
            ..default()
        },
        DebugFpsText
    ));
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // .add_systems(Startup, setup_fps_text)
        // .add_systems(Update, (debug_fps, debug_camera));
    }
}
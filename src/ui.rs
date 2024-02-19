use bevy::prelude::*;

use crate::{health::Health, score::Score};

#[derive(Component)]
struct Ui;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, build_playing_ui);
        app.add_systems(Update, update_ui);
    }
}

fn update_ui(mut commands: Commands, mut query: Query<&mut Text>, score: Res<Score>) {
    for mut text in query.iter_mut() {
        text.sections = vec![TextSection {
            value: format!("Score: {}", score.value),
            ..default()
        },
        TextSection {
            value: "Health: 0".to_string(),
            ..default()
        }
        // TextSection {
        //     value: format!("Health: {}", health.value)
        // }
        ];
    }
}
fn build_game_ui(mut commands: Commands) {
    commands.spawn(ButtonBundle {
        ..default()
    });
}
fn build_playing_ui(mut commands: Commands) {
    commands.spawn(TextBundle::from_section("Hello World", TextStyle::default()));
}
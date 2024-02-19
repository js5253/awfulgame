use bevy::prelude::*;
use derive_new::new;

#[derive(PartialEq, Eq, Hash, Event, Debug)]
pub enum ScoreChange {
    Increment,
    Decrement,
}
#[derive(Component)]
struct ScoreText;
#[derive(Resource, Debug, Default)]
pub struct Score {
    pub value: i32,
}

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            // .add_systems(PostStartup, set_up_score)
            .add_systems(Update, (listen_for_changes));
    }
}

fn set_up_score(commands: &mut Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section("Hello World", TextStyle::default()),
        ..default()
    });
}
fn listen_for_changes(
    mut score: ResMut<Score>,
    mut score_change_reader: EventReader<ScoreChange>,
) {
    for event in score_change_reader.read() {
        score.value += 1;
        println!("Another point! Now {}", score.value);
    }
}
fn increment_score(mut score: ResMut<Score>) {
    score.value += 1;
}

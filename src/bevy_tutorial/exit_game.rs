use bevy::{app::AppExit, prelude::*};

#[derive(Event)]
pub struct GameOverEvent {
    pub final_score: u32,
}

// this fires on startup for some reason
// TODO design some system to make sure that this does not happen anymore
// Maybe raise a github issue
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut _app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("App exit");
        // _app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over_event(mut game_over_event_reader: EventReader<GameOverEvent>) {
    // multiple systems can send the same event in the same frame
    for go_event in game_over_event_reader.iter() {
        println!("Your final score is  : {}", go_event.final_score)
    }
}

use bevy::prelude::*;
use battlyspace::GameState;

/// Custom game plugin for all things on the Pause screen
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            // TODO
            // .add_systems(OnEnter(GameState::Pause), (load_rock_timer, spawn_score_counter))
            .add_systems(Update, check_unpause.run_if(in_state(GameState::Pause)));
    }
}

/// Checks if Esc or P has been pressed to switch to GameState::Game to resume game
fn check_unpause(
    mut game_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.any_just_pressed([KeyCode::Escape, KeyCode::P]) {
        game_state.set(GameState::Game);
    }
}
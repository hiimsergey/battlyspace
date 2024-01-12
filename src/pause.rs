use bevy::prelude::*;
use battlyspace::*;

/// Custom game plugin for all things on the Pause screen
pub struct PausePlugin;

/// Tag component for entities added on the menu screen
#[derive(Component)]
pub struct OnPauseScreen;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Pause), spawn_pause_text)
            .add_systems(Update, check_unpause.run_if(in_state(GameState::Pause)))
            .add_systems(OnExit(GameState::Pause), cleanup::<OnPauseScreen>);
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

/// Spawns a text reading "Pause" in the Pause screen
fn spawn_pause_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "Pause",
            HEADING_SIZE,
            Color::SILVER,
            HEADING_Y
        ),
        OnPauseScreen
    ));
}
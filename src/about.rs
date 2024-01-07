use bevy::prelude::*;
use battlyspace::*;

/// Custom game plugin for all things on the About screen
pub struct AboutPlugin;

/// Tag component for entities added on the About screen
#[derive(Component)]
struct OnAboutScreen;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::About),
                (spawn_about_text, cleanup::<Ship>)
            )
            .add_systems(
                Update,
                (about_input, rotate_text).run_if(in_state(GameState::About))
            )
            .add_systems(OnExit(GameState::About), cleanup::<OnAboutScreen>);
    }
}

/// Prints text seen on About screen: heading, info, input hint
fn spawn_about_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "About the game",
            HEADING_SIZE,
            ABOUT_TEXT_COLOR,
            HEADING_Y
        ),
        OnAboutScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            ABOUT_TEXT,
            ABOUT_TEXT_SIZE,
            ABOUT_TEXT_COLOR,
            0.
        ),
        OnAboutScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press X to exit",
            INPUT_HINT_SIZE,
            ABOUT_TEXT_COLOR,
            INPUT_HINT_ONE_Y
        ),
        OnAboutScreen, TextRotation
    ));
}

/// Checks for use input (spacebar) to launch main menu
fn about_input(
    mut game_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.just_pressed(KeyCode::X) {
        game_state.set(GameState::Menu);
    }
}
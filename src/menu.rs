use bevy::prelude::*;
use battlyspace::*;

/// Custom game plugin für all things on the menu screen
pub struct MenuPlugin;

/// Tag component for entities added on the menu screen
#[derive(Component)]
struct OnMenuScreen;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), (spawn_menu_text, spawn_ship))
            .add_systems(
                Update,
                (lobby_input, rotate_ship, rotate_text)
                    .run_if(in_state(GameState::Menu))
            )
            .add_systems(OnExit(GameState::Menu), cleanup::<OnMenuScreen>);
    }
}

/// Rotates the ship in the Menu screen
fn rotate_ship(
    mut ship_query: Query<&mut Transform, With<Ship>>,
    time: Res<Time>
) {
    ship_query.single_mut().rotate_z(-0.5 * time.delta_seconds());
}

/// Spawns the Ship entity at the center of the screen
///
/// The ship will be shown in the menu screen and in-game
pub fn spawn_ship(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: assets.load("sprites/ship.png"),
            transform: Transform::from_xyz(0., 0., 1.)
                .with_scale(Vec3::splat(3.)),
            ..default()
        },
        Ship {
            impulse_direction: Vec3::splat(0.),
            movement_speed: 50., // TODO look at the equation, why does the value need to be so high?
            rotation_speed: 0.,
        }
    ));
}

/// Spawns the game title and keyboard input hints
fn spawn_menu_text(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(
            &assets,
            "Battlyspace",
            HEADING_SIZE,
            Color::WHITE,
            HEADING_Y
        ),
        OnMenuScreen
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press X to start",
            INPUT_HINT_SIZE,
            Color::WHITE,
            INPUT_HINT_UPPER_Y
        ),
        OnMenuScreen, TextRotation
    ));
    commands.spawn((
        text_from_str(
            &assets,
            "Press A for About",
            INPUT_HINT_SIZE,
            Color::WHITE,
            INPUT_HINT_LOWER_Y
        ),
        OnMenuScreen, TextRotation
    ));
}
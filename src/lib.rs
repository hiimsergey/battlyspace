//! [Another](https://github.com/hiimsergey/flappyspace) small 2D-game built with [Bevy](https://bevyengine.org)
//! - [GitHub](https://github.com/hiimsergey/battlyspace)

use bevy::prelude::*;

// Imports constants
mod consts;
pub use consts::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    /// Main menu, first/default game state
    #[default]
    Menu,
    /// About page, info and credits
    About,
    /// In-game screen,
    Game,
    /// Pause screen for game
    Pause,
    /// Game Over screen
    Crashed
}


/// Player component with variable velocity
#[derive(Component)]
pub struct Ship {
    pub impulse_direction: Vec3,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

/// Component for bullets shot by ship
#[derive(Component)]
pub struct Bullet;

/// Component for rocks spawned around the game area
#[derive(Component)]
pub struct Rock {
    pub hp: u8,
    pub speed: f32
}


/// Tag component for rotating components
#[derive(Component)]
pub struct TextRotation;

/// Component for tracking the game score
#[derive(Component)]
pub struct Scoreboard {
    pub score: usize
}

/// Resource for storing the highscore
#[derive(Resource)]
pub struct Highscore(pub usize);

/// Timer resource for rock spawning
#[derive(Resource, Deref,DerefMut)]
pub struct RockTimer(pub Timer);


/// Despawns all entities for a component
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// TODO change this desciprtions after UI update
/// Checks for user input, either launches game or About screen
///
/// Is applied in Menu and Game Over screen
pub fn lobby_input(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    assets: Res<AssetServer>,
    key: Res<Input<KeyCode>>
) {
    // If user presses Esc, quits game
    if key.any_just_pressed([KeyCode::Escape, KeyCode::Q]) { std::process::exit(0); }

    // If user presses X, the game launches
    if key.just_pressed(KeyCode::X) {
        play_sound(&mut commands, &assets, "start");
        game_state.set(GameState::Game);
    }

    // If user presses A, the About screen launches
    if key.just_pressed(KeyCode::A) {
        // TODO make about screen sound
        game_state.set(GameState::About);
    }
}

/// Plays sound at ./assets/sounds/`sound`.ogg by spawning AudioBundle
pub fn play_sound(commands: &mut Commands, assets: &Res<AssetServer>, sound: &str) {
    commands.spawn(
        AudioBundle {
            source: assets.load(format!("sounds/{sound}.ogg")),
            settings: PlaybackSettings::DESPAWN
        }
    );
}

/// Rotates every text field entity of the Rotation component
pub fn rotate_text(
    mut text_query: Query<&mut Transform, With<TextRotation>>,
    time: Res<Time>
) {
    for mut transform in text_query.iter_mut() {
        transform.rotation =
            Quat::from_rotation_z(time.elapsed_seconds().cos()) / 2.;
    }
}

/// Returns a Text2dBundle to be spawned
pub fn text_from_str(
    assets: &Res<AssetServer>,
    text: &str,
    font_size: f32,
    color: Color,
    text_y: f32
) -> Text2dBundle {
    let text_style = TextStyle {
        font: assets.load("fonts/PixelifySans-SemiBold.ttf"),
        font_size,
        color
    };

    Text2dBundle {
        text: Text::from_section(text, text_style)
            .with_alignment(TextAlignment::Center),
        transform: Transform::from_xyz(0., text_y, 1.),
        ..default()
    }
}
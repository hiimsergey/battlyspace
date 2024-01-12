// TODO FEATURE settings screen
    // rocks
        // spawn periods
        // randomize size?
        // (avg) size (HP)
        // randomize speed?
        // (avg) speed
    // bullets
        // how long the periods are until you get new bullets
        // how many bullets you get
// TODO FEATURE make the rocks get faster with time
// TODO FEATURE custom texture for low hp
// TODO FEATURE add non-stop shooting if key.pressed(KeyCode::Space)
// TODO END how to embed resources
// TODO END how to add a window icon
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution};

use battlyspace::{BOUNDS, GameState};

mod menu; mod about; mod game; mod pause; mod crashed;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                // Enables "Pixel Perfect" to prevent bluriness
                .set(ImagePlugin::default_nearest())
                
                // Window settings
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        title: "Battlyspace".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(BOUNDS.x, BOUNDS.y),
                        ..default()
                    }),
                    ..default()
                })
                .build(),

            // Game plugins
            menu::MenuPlugin,
            about::AboutPlugin,
            game::GamePlugin,
            pause::PausePlugin,
            crashed::CrashedPlugin
        ))

        // Adds black background
        .insert_resource(ClearColor(Color::BLACK))

        // Declares game state, set to default (Menu)
        .add_state::<GameState>()

        // Startup logic
        .add_systems(Startup, setup)

        .run();
}

/// First system to run
///
/// Spawns default 2D camera bundle
fn setup(
    mut commands: Commands
) {
    // Spawns default camera setup
    commands.spawn(Camera2dBundle::default());
}
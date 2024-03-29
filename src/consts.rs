use bevy::prelude::{Color, KeyCode, Vec2};

/* ------------------------------- GAME FIELD ------------------------------- */

/// Bounds within which the ship can move
pub const BOUNDS: Vec2 = Vec2::splat(800.); // TODO get sane values

/// Radius describing the circle on which new rocks are spawned with (0, 0, 0)
/// being the cneter
pub const RADIUS: f32 = BOUNDS.x * 0.5 + 128.; // TODO VALUE


/* ----------------------------- KEY BINDINGS ------------------------------- */

/// Keys that trigger forward movement
pub const KEYS_FORWARD: [KeyCode; 3] = [KeyCode::Up, KeyCode::W, KeyCode::K];

/// Keys that trigger turning left/counterclockwise
pub const KEYS_TURN_LEFT: [KeyCode; 3] = [KeyCode::Left, KeyCode::A, KeyCode::H];

/// Keys that trigger turning right/clockwise
pub const KEYS_TURN_RIGHT: [KeyCode; 3] = [KeyCode::Right, KeyCode::D, KeyCode::L];

/// Keys that trigger shooting
pub const KEYS_SHOOT: [KeyCode; 2] = [KeyCode::Space, KeyCode::F];

/// Keys that trigger Pause
pub const KEYS_PAUSE: [KeyCode; 4] =
    [KeyCode::Escape, KeyCode::ShiftLeft, KeyCode::ShiftRight, KeyCode::P];


/* -------------------------------- PHYSICS --------------------------------- */

/// Speed of the bullets
pub const BULLET_VELOCITY: f32 = 10.; // TODO VALUE

/// Speed of the rocks
pub const ROCK_VELOCITY: f32 = 1.; // TODO VALUE


/* ------------------------------- TEXT BOXES ------------------------------- */

/// Size of heading text
pub const HEADING_SIZE: f32 = 100.;

/// Y-coordinate of heading text
pub const HEADING_Y: f32 = 200.;


/// Y-coordinate of highscore text
pub const HIGHSCORE_Y: f32 = HEADING_Y - 50.;


/// Size of input hints ("Press \<key\> to \<action\>")
pub const INPUT_HINT_SIZE: f32 = 50.;

/// Y-coordinate of input hint ("Press \<key\> to \<action\>"), if there is one
pub const INPUT_HINT_ONE_Y: f32 = -220.;

/// Y-coordinate of upper input hint ("Press \<key\> to \<action\>"), if there are two
pub const INPUT_HINT_UPPER_Y: f32 = -170.;

/// Y-coordinate of lower input hint ("Press \<key\> to \<action\>"), if there are two
pub const INPUT_HINT_LOWER_Y: f32 = -270.;


/// Text you're going to see on the About screen
pub const ABOUT_TEXT: &str = "Battlyspace
<https://github.com/hiimsergey/battlyspace>

Built with Bevy Engine
<https://bevyengine.org>

v0.0.1 GPL-3.0 License";

/// Size of content on the About screen
pub const ABOUT_TEXT_SIZE: f32 = 30.;

/// Color of the text on the About screen
pub const ABOUT_TEXT_COLOR: Color = Color::YELLOW;
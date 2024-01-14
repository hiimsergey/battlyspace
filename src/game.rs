// TODO ADD despawn rocks after they moved outside of bounds
use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use fastrand;
use battlyspace::*;

/// Custom game plugin for all things in-game
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), (load_rock_timer, spawn_score_counter))
            .add_systems(
                Update,
                (
                    idle_ship_impulse,

                    spawn_rocks,
                    update_bullets,
                    update_rocks,
                    update_ship,

                    check_bullet_collisions,
                    check_ship_collisions,
                    
                    check_pause
                )
                .run_if(in_state(GameState::Game))
            )
            .add_systems(OnExit(GameState::Game), cleanup::<Scoreboard>);
    }
}

/// Looks for collisions between bullets and rocks and alters the former accordingly
fn check_bullet_collisions(
    mut commands: Commands,
    mut score_query: Query<&mut Scoreboard>,
    mut score_text_query: Query<&mut Text, With<Scoreboard>>,
    mut rock_query: Query<(Entity, &Transform, &mut Rock)>,
    assets: Res<AssetServer>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (rock_entity, rock_transform, mut rock) in &mut rock_query {
            if collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate() * 10.5, // TODO VALUE
                rock_transform.translation,
                // TODO COMMENT
                rock_transform.scale.truncate() * 11.5
            ).is_some() {
                commands.entity(bullet_entity).despawn();
                rock.hp -= 1;
                if rock.hp == 0 {
                    let mut scoreboard = score_query.single_mut();
                    scoreboard.score += 1;
                    score_text_query.single_mut().sections[0].value =
                        scoreboard.score.to_string();
                    play_sound(&mut commands, &assets, "crash");
                    commands.entity(rock_entity).despawn();
                }
            }
        }
    }
}

/// Checks if Esc or P has been pressed to launch Pause screen
fn check_pause(
    mut game_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.any_just_pressed(KEYS_PAUSE) {
        game_state.set(GameState::Pause);
    }
}

/// Looks for collisions between a rock and the ship and launches the Game Over screen
fn check_ship_collisions(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    assets: Res<AssetServer>,
    rock_query: Query<&Transform, With<Rock>>,
    ship_query: Query<&Transform, With<Ship>>
) {
    let ship_transform = ship_query.single();

    for rock_transform in &rock_query {
        if collide(
            rock_transform.translation,
            // TODO COMMENT
            rock_transform.scale.truncate() * 11.5,
            ship_transform.translation,
            ship_transform.scale.truncate() * 10.5 // TODO VALUE
        ).is_some() {
            play_sound(&mut commands, &assets, "crash");
            game_state.set(GameState::Crashed);
        }
    }
}

/// Makes the ship slowly move toward its recent direction when idle to make it
/// look like it's actually in space
fn idle_ship_impulse(mut query: Query<(&mut Transform, &Ship)>, time: Res<Time>) {
    let (mut transform, ship) = query.single_mut();
    transform.translation += 50. * ship.impulse_direction * time.delta_seconds(); // TODO VALUE
}

/// Repeatedly loads RockTimer to queue new rocks to be spawned
fn load_rock_timer(mut commands: Commands) {
    commands.insert_resource(RockTimer(Timer::from_seconds(
        1.,
        TimerMode::Repeating
    )));
}

/// Makes the ship shoot a bullet out of its front moving with the same
/// direction as the ship at that moment
fn shoot(commands: &mut Commands, assets: &Res<AssetServer>, transform: &mut Transform) {
    commands.spawn((
        SpriteBundle {
            texture: assets.load("sprites/bullet.png"),
            transform: transform.with_scale(Vec3::splat(3.)),
            ..default()
        },
        Bullet
    ));
}

/// Repeatedly spawns rocks around the game area
fn spawn_rocks(
    mut commands: Commands,
    mut timer: ResMut<RockTimer>,
    assets: Res<AssetServer>,
    time: Res<Time>
) {
    if timer.tick(time.delta()).finished() {
        // Random angle in radians for the direction the rock will spawn from
        let random_angle = fastrand::f32() * 2. * PI;

        commands.spawn((
            SpriteBundle {
                texture: assets.load("sprites/rock1.png"),
                // Calculates the position on the circle using RADIUS and
                // cos(angle) and sin(angle), respectively
                transform: Transform::from_xyz(
                    RADIUS * f32::cos(random_angle),
                    RADIUS * f32::sin(random_angle),
                    0.
                )
                    // For some reason the rock floats at +90° of what I wanted
                    // + 0.5 * PI seems to help though
                    .with_rotation(
                        Quat::from_rotation_z(random_angle + 0.5 * PI)
                    )
                    .with_scale(Vec3::splat(3.)),
                ..default()
            },
            Rock { hp: fastrand::u8(1..=5), speed: ROCK_VELOCITY } // TODO VALUE
        ));
    }
}

/// TODO
/// Spawns score counter as a big number in-game
fn spawn_score_counter(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        text_from_str(&assets, "0", HEADING_SIZE, Color::WHITE, HEADING_Y),
        Scoreboard { score: 0 }
    ));
}

/// Controls the movement of bullets
pub fn update_bullets(mut query: Query<&mut Transform, With<Bullet>>) {
    for mut transform in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        transform.translation += movement_direction * BULLET_VELOCITY;
    }
}

/// Controls the movement of rocks
pub fn update_rocks(mut query: Query<(&mut Transform, &mut Rock)>) {
    for (mut transform, mut rock) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        transform.translation += movement_direction * rock.speed;

        // TODO if settings.accelerate ...
        rock.speed += 0.01;
    }
}

/// Controls the movement and rotation of the ship
fn update_ship(
    mut commands: Commands,
    mut quey: Query<(&mut Transform, &mut Ship)>,
    assets: Res<AssetServer>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut transform, mut ship) = quey.single_mut();

    if ship.rotation_speed.abs() > 0.5 { ship.rotation_speed *= 0.8; } // TODO VALUE
    if ship.movement_speed > 50. { ship.movement_speed *= 0.9; }
    
    if key.any_pressed(KEYS_TURN_LEFT) { ship.rotation_speed += 0.5 }
    if key.any_pressed(KEYS_TURN_RIGHT) { ship.rotation_speed -= 0.5 }
    
    if key.any_just_pressed(KEYS_SHOOT) {
        shoot(&mut commands, &assets, &mut transform);
        play_sound(&mut commands, &assets, "shoot");
    }

    // Rotates the ship around the z-axis
    transform.rotate_z(ship.rotation_speed * time.delta_seconds());

    // Gets the ship's forward vector by applying the current rotation to the
    // ship's initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;

    // Gets the distance the ship will move based on direction, the ship's
    // movement speed and delta time
    let movement_distance = ship.movement_speed * time.delta_seconds();

    if key.any_pressed(KEYS_FORWARD) {
        if ship.movement_speed < 500. {
            ship.movement_speed *= 1.2;
        }

        // Updates the ship translation 
        transform.translation += movement_direction * movement_distance;

        // TODO COMMENT
        ship.impulse_direction = movement_direction;
    }

    // Bounds the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2., 0.));
    transform.translation = transform.translation.min(extents).max(-extents);
}

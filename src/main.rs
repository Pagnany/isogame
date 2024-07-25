use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod mapgenerator;
pub mod player;
pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 50.0;

pub const MOVESPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "isogame".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                move_player_middle,
                system::kill_game_on_esc,
                system::fps_update_system,
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(TICK_TIME))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                ..Default::default()
            }),
        ]),
        system::FpsText,
    ));

    //crate::mapgenerator::create_test_map(&mut commands, &asset_server);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_left_01.png"),
            transform: Transform::from_xyz(-50.0, 0.0, 0.0),
            ..default()
        },
        player::Player { left_hand: true },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_right_01.png"),
            transform: Transform::from_xyz(50.0, 0.0, 0.0),
            ..default()
        },
        player::Player { left_hand: false },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_middle_01.png"),
            transform: Transform::from_xyz(50.0, 0.0, 0.0),
            ..default()
        },
        player::PlayerMiddle,
    ));
}

struct PlayerPos {
    x: f32,
    y: f32,
}

fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&player::Player, &mut Transform)>,
) {
    let mut player_left = PlayerPos { x: 0.0, y: 0.0 };
    let mut player_right = PlayerPos { x: 0.0, y: 0.0 };
    // get new values from key input
    for (player, transform) in &mut query {
        if player.left_hand {
            player_left.x = transform.translation.x;
            player_left.y = transform.translation.y;
            if keys.pressed(KeyCode::KeyW) {
                player_left.y += MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyA) {
                player_left.x -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyS) {
                player_left.y -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyD) {
                player_left.x += MOVESPEED * time.delta_seconds();
            }
        } else {
            player_right.x = transform.translation.x;
            player_right.y = transform.translation.y;
            if keys.pressed(KeyCode::KeyU) {
                player_right.y += MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyH) {
                player_right.x -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyJ) {
                player_right.y -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyK) {
                player_right.x += MOVESPEED * time.delta_seconds();
            }
        }
    }

    // set new value
    for (player, mut transform) in &mut query {
        if player.left_hand {
            if player_left.x < SCREEN_WIDTH / 2.0 * -1.0 {
                player_left.x = SCREEN_WIDTH / 2.0 * -1.0;
            } else if player_left.x > SCREEN_WIDTH / 2.0 {
                player_left.x = SCREEN_WIDTH / 2.0;
            }
            if player_left.y < SCREEN_HEIGHT / 2.0 * -1.0 {
                player_left.y = SCREEN_HEIGHT / 2.0 * -1.0;
            } else if player_left.y > SCREEN_HEIGHT / 2.0 {
                player_left.y = SCREEN_HEIGHT / 2.0;
            }
            transform.translation.x = player_left.x;
            transform.translation.y = player_left.y;
        } else {
            if player_right.x < SCREEN_WIDTH / 2.0 * -1.0 {
                player_right.x = SCREEN_WIDTH / 2.0 * -1.0;
            } else if player_right.x > SCREEN_WIDTH / 2.0 {
                player_right.x = SCREEN_WIDTH / 2.0;
            }
            if player_right.y < SCREEN_HEIGHT / 2.0 * -1.0 {
                player_right.y = SCREEN_HEIGHT / 2.0 * -1.0;
            } else if player_right.y > SCREEN_HEIGHT / 2.0 {
                player_right.y = SCREEN_HEIGHT / 2.0;
            }
            transform.translation.x = player_right.x;
            transform.translation.y = player_right.y;
        }
    }
}

fn move_player_middle(
    mut query_middle: Query<&mut Transform, (With<player::PlayerMiddle>, Without<player::Player>)>,
    query_players: Query<&Transform, (With<player::Player>, Without<player::PlayerMiddle>)>,
) {
    let player_1x = query_players.iter().nth(0).unwrap().translation.x;
    let player_1y = query_players.iter().nth(0).unwrap().translation.y;
    let player_2x = query_players.iter().nth(1).unwrap().translation.x;
    let player_2y = query_players.iter().nth(1).unwrap().translation.y;
    let player_middle_x = (player_1x + player_2x) / 2.0;
    let player_middle_y = (player_1y + player_2y) / 2.0;

    for mut transform in &mut query_middle {
        transform.translation.x = player_middle_x;
        transform.translation.y = player_middle_y;
    }
}

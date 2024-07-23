use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};

pub mod mapgenerator;
pub mod player;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

const TICK_TIME: f32 = 1.0 / 50.0;

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "isogame".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(
            FixedUpdate,
            (
                fps_update_system,
                player_movement_system,
                move_tiles,
                kill_game_on_esc,
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(TICK_TIME.into()))
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
        FpsText,
    ));

    crate::mapgenerator::create_test_map(&mut commands, &asset_server);

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
}

fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&player::Player, &mut Transform)>,
) {
    let move_speed = 200.0;
    for (player, mut transform) in &mut query {
        if player.left_hand {
            if keys.pressed(KeyCode::KeyW) {
                transform.translation.y += move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyA) {
                transform.translation.x -= move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyS) {
                transform.translation.y -= move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyD) {
                transform.translation.x += move_speed * time.delta_seconds();
            }
        } else {
            if keys.pressed(KeyCode::KeyI) {
                transform.translation.y += move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyJ) {
                transform.translation.x -= move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyK) {
                transform.translation.y -= move_speed * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyL) {
                transform.translation.x += move_speed * time.delta_seconds();
            }
        }
    }
}

fn move_tiles(
    mut query_tiles: Query<&mut Transform, (With<mapgenerator::MapTile>, Without<player::Player>)>,
    query_players: Query<&Transform, (With<player::Player>, Without<mapgenerator::MapTile>)>,
) {
    let player_1x = query_players.iter().nth(0).unwrap().translation.x;
    let player_1y = query_players.iter().nth(0).unwrap().translation.y;
    let player_2x = query_players.iter().nth(1).unwrap().translation.x;
    let player_2y = query_players.iter().nth(1).unwrap().translation.y;
    let player_middle_x = (player_1x + player_2x) / 2.0;
    let player_middle_y = (player_1y + player_2y) / 2.0;

    for mut transform in &mut query_tiles {
        transform.translation.x = player_middle_x;
        transform.translation.y = player_middle_y;
    }
}

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn kill_game_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

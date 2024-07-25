use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod player;
pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 50.0;

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
                player::player_movement_system,
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

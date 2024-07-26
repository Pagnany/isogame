use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod collision;
pub mod enemy;
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

    // enemy
    // under
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_under_01.png"),
            transform: Transform::from_xyz(-250.0, 30.0, 0.0),
            ..default()
        },
        enemy::Enemy {
            under_water: enemy::EnemyType::UnderWater,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_under_01.png"),
            transform: Transform::from_xyz(150.0, -100.0, 0.0),
            ..default()
        },
        enemy::Enemy {
            under_water: enemy::EnemyType::UnderWater,
        },
    ));
    // over
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_over_01.png"),
            transform: Transform::from_xyz(170.0, -200.0, 0.001),
            ..default()
        },
        enemy::Enemy {
            under_water: enemy::EnemyType::AboveWater,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_over_01.png"),
            transform: Transform::from_xyz(-470.0, -200.0, 0.001),
            ..default()
        },
        enemy::Enemy {
            under_water: enemy::EnemyType::AboveWater,
        },
    ));

    // player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_left_01.png"),
            transform: Transform::from_xyz(-50.0, 0.0, 0.9),
            ..default()
        },
        player::Player { left_hand: true },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_right_01.png"),
            transform: Transform::from_xyz(50.0, 0.0, 0.9),
            ..default()
        },
        player::Player { left_hand: false },
    ));

    // test middle point
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_middle_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        player::PlayerMiddle,
    ));
}

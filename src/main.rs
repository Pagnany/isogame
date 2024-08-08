use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod collision;
pub mod enemy;
pub mod menu;
pub mod physics;
pub mod player;
pub mod system;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 50.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    MainMenu,
    InGame,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MainMenuSet;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "isogame".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin,
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.insert_state(GameState::MainMenu);
    app.add_systems(
        FixedUpdate,
        (
            (
                collision::player_with_enemy_over,
                collision::middle_with_enemy_under,
                collision::player_with_player,
                player::player_velocity_input_system,
                physics::player_velocity_system,
                enemy::enemy_movement_system,
            )
                .in_set(GameplaySet),
            (menu::button_system).in_set(MainMenuSet),
            system::kill_game_on_esc,
            system::fps_update_system,
        ),
    );
    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(GameState::MainMenu), menu::spawn_main_menu);
    app.add_systems(OnEnter(GameState::InGame), spawn_ingame);
    app.add_systems(OnExit(GameState::MainMenu), menu::despawn_main_menu);
    app.add_systems(OnExit(GameState::GameOver), menu::despawn_main_menu);
    app.add_systems(
        OnEnter(GameState::GameOver),
        (menu::spawn_gameover_menu, despawn_ingame),
    );
    app.configure_sets(
        FixedUpdate,
        (
            GameplaySet.run_if(in_state(GameState::InGame)),
            MainMenuSet
                .run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::GameOver))),
        ),
    );

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // background
    commands.spawn((SpriteBundle {
        texture: asset_server.load("textures/background_01.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));

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
}

#[derive(Component)]
struct InGameEntity;

fn spawn_ingame(mut commands: Commands, asset_server: Res<AssetServer>) {
    // enemy
    // under
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_under_01.png"),
            transform: Transform::from_xyz(-250.0, 30.0, 0.0),
            ..default()
        },
        enemy::Enemy {
            height: 50.0,
            width: 50.0,
            inital_pos: Vec2 { x: -250.0, y: 30.0 },
            enemy_type: enemy::EnemyType::UnderWater,
            ..default()
        },
        InGameEntity,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_under_01.png"),
            transform: Transform::from_xyz(150.0, -100.0, 0.0),
            ..default()
        },
        enemy::Enemy {
            height: 50.0,
            width: 50.0,
            inital_pos: Vec2 {
                x: 150.0,
                y: -100.0,
            },
            enemy_type: enemy::EnemyType::UnderWater,
            ..default()
        },
        InGameEntity,
    ));
    // over
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_over_01.png"),
            transform: Transform::from_xyz(170.0, -200.0, 0.001),
            ..default()
        },
        enemy::Enemy {
            height: 50.0,
            width: 50.0,
            inital_pos: Vec2 {
                x: 170.0,
                y: -200.0,
            },
            enemy_type: enemy::EnemyType::AboveWater,
            ..default()
        },
        InGameEntity,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/enemy_over_01.png"),
            transform: Transform::from_xyz(-470.0, -200.0, 0.001),
            ..default()
        },
        enemy::Enemy {
            height: 50.0,
            width: 50.0,
            inital_pos: Vec2 {
                x: -470.0,
                y: -200.0,
            },
            enemy_type: enemy::EnemyType::AboveWater,
            ..default()
        },
        InGameEntity,
    ));

    // player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_left_01.png"),
            transform: Transform::from_xyz(-50.0, 0.0, 0.9),
            ..default()
        },
        player::Player {
            left_hand: true,
            velocity: Vec2::ZERO,
        },
        InGameEntity,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_right_01.png"),
            transform: Transform::from_xyz(50.0, 0.0, 0.9),
            ..default()
        },
        player::Player {
            left_hand: false,
            velocity: Vec2::ZERO,
        },
        InGameEntity,
    ));

    // test middle point
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player_middle_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        player::PlayerMiddle,
        InGameEntity,
    ));
}

fn despawn_ingame(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

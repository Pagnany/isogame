use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod collision;
pub mod enemy;
pub mod menu;
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
                player::player_movement_system,
            )
                .in_set(GameplaySet),
            (menu::button_system).in_set(MainMenuSet),
            system::kill_game_on_esc,
            system::fps_update_system,
        ),
    );
    app.add_systems(Startup, setup);
    app.add_systems(OnExit(GameState::MainMenu), menu::despawn_main_menu);
    app.configure_sets(
        FixedUpdate,
        (
            GameplaySet.run_if(in_state(GameState::InGame)),
            MainMenuSet.run_if(in_state(GameState::MainMenu)),
        ),
    );

    app.run();
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
            height: 50.0,
            width: 50.0,
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
            height: 50.0,
            width: 50.0,
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
            height: 50.0,
            width: 50.0,
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
            height: 50.0,
            width: 50.0,
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

    // Button
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: menu::NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("fonts/SuperBubble-Rpaj3.ttf"),
                            font_size: 20.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

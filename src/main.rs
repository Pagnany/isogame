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
        .add_systems(FixedUpdate, (fps_update_system, move_tiles))
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
}

fn move_tiles(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<crate::mapgenerator::MapTile>>,
) {
    let move_speed = 150.;
    for mut transform in &mut query {
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

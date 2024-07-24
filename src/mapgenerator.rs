use bevy::prelude::*;

#[derive(Component)]
pub struct MapTile;

const TILE_WIDTH: f32 = 50.0;
const TILE_HIGHT: f32 = 50.0;
const TILE_COLUMNS: i32 = 10;
const TILE_ROWS: i32 = 10;

pub fn create_test_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let left = 0.0 - crate::SCREEN_WIDTH / 2.0 + TILE_WIDTH / 2.0;
    let bottom = 0.0 - crate::SCREEN_HEIGHT / 2.0 + TILE_HIGHT / 2.0;
    let mut x = left;
    let mut y = bottom;
    for _ in 0..TILE_COLUMNS {
        for _ in 0..TILE_ROWS {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/texture_ground_01.png"),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                MapTile,
            ));
            x += TILE_WIDTH;
        }
        x = left;
        y += TILE_HIGHT;
    }
}

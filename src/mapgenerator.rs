use bevy::prelude::*;

const TILE_WIDTH: f32 = 50.0;
const TILE_HIGHT: f32 = 50.0;

pub fn create_test_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let left = 0.0 - crate::SCREEN_WIDTH / 2.0 + TILE_WIDTH / 2.0;
    let bottom = 0.0 - crate::SCREEN_HEIGHT / 2.0 + TILE_HIGHT / 2.0;
    let mut x = left;
    let mut y = bottom;
    for _ in 0..10 {
        for _ in 0..10 {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("textures/texture_ground_01.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
            x += TILE_WIDTH;
        }
        x = left;
        y += TILE_HIGHT;
    }
}

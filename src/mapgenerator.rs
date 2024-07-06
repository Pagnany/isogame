use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const TILE_WIDTH: f32 = 50.0;
const TILE_HIGHT: f32 = 50.0;

pub fn create_test_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut x = 0.0;
    let mut y = 0.0;
    for _ in 0..10 {
        for _ in 0..10 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HIGHT))),
                material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
            x += TILE_WIDTH + 5.0;
        }
        x = 0.0;
        y += TILE_HIGHT + 5.0;
    }
}

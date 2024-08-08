use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq, Default)]
pub enum EnemyType {
    UnderWater,
    #[default]
    AboveWater,
}

#[derive(Component, Default)]
pub struct Enemy {
    pub height: f32,
    pub width: f32,
    pub inital_pos: Vec2,
    pub enemy_type: EnemyType,
    pub movement_factor: Vec2,
}

pub fn enemy_movement_system(time: Res<Time>, mut query: Query<(&mut Enemy, &mut Transform)>) {
    let mut rng = rand::thread_rng();

    for (mut enemy, mut transform) in query.iter_mut() {
        if enemy.enemy_type == EnemyType::AboveWater {
            let mut y = enemy.inital_pos.y;
            let mut x = enemy.inital_pos.x;

            enemy.movement_factor.y += 6.0 * rng.gen::<f32>() * time.delta_seconds();
            enemy.movement_factor.x += 6.0 * rng.gen::<f32>() * time.delta_seconds();
            y += (enemy.movement_factor.y).sin() * 3.0;
            x += (enemy.movement_factor.x).sin() * 3.0;

            transform.translation.y = y;
            transform.translation.x = x;
        }
    }
}

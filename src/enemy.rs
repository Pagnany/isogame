use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq)]
pub enum EnemyType {
    UnderWater,
    AboveWater,
}

#[derive(Component)]
pub struct Enemy {
    pub height: f32,
    pub width: f32,
    pub inital_pos: Vec2,
    pub under_water: EnemyType,
    pub movement_factor: f32,
}

pub fn enemy_movement_system(time: Res<Time>, mut query: Query<(&mut Enemy, &mut Transform)>) {
    let mut rng = rand::thread_rng();

    for (mut enemy, mut transform) in query.iter_mut() {
        let mut y = enemy.inital_pos.y;

        enemy.movement_factor += 3.0 * time.delta_seconds();
        y += (enemy.movement_factor).sin() * 3.0;

        transform.translation.y = y;
    }
}

use bevy::prelude::*;

#[derive(PartialEq, Eq)]
pub enum EnemyType {
    UnderWater,
    AboveWater,
}

#[derive(Component)]
pub struct Enemy {
    pub height: f32,
    pub width: f32,
    pub under_water: EnemyType,
}

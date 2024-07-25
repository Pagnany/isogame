use bevy::prelude::*;

pub enum EnemyType {
    UnderWater,
    AboveWater,
}

#[derive(Component)]
pub struct Enemy {
    pub under_water: EnemyType,
}

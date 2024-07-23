use bevy::prelude::*;

pub const PLAYER_HEIGHT: f32 = 50.0;
pub const PLAYER_WIDTH: f32 = 30.0;

#[derive(Component)]
pub struct Player {
    pub left_hand: bool,
}

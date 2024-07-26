use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;

pub const PLAYER_HIGHT: f32 = 50.0;
pub const PLAYER_WIDTH: f32 = 50.0;

pub const MOVESPEED: f32 = 200.0;
pub const LINE_LENGTH: f32 = 500.0;
pub const BONK_DISTANCE: f32 = 50.0;

#[derive(Component)]
pub struct Player {
    pub left_hand: bool,
}

#[derive(Component)]
pub struct PlayerMiddle;

struct PlayerPos {
    x: f32,
    y: f32,
}

pub fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
    mut query_middle: Query<&mut Transform, (With<PlayerMiddle>, Without<Player>)>,
) {
    let mut player_left = PlayerPos { x: 0.0, y: 0.0 };
    let mut player_right = PlayerPos { x: 0.0, y: 0.0 };
    // get new values from key input
    for (player, transform) in &mut query {
        if player.left_hand {
            player_left.x = transform.translation.x;
            player_left.y = transform.translation.y;
            if keys.pressed(KeyCode::KeyW) {
                player_left.y += MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyA) {
                player_left.x -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyS) {
                player_left.y -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyD) {
                player_left.x += MOVESPEED * time.delta_seconds();
            }
        } else {
            player_right.x = transform.translation.x;
            player_right.y = transform.translation.y;
            if keys.pressed(KeyCode::KeyU) {
                player_right.y += MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyH) {
                player_right.x -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyJ) {
                player_right.y -= MOVESPEED * time.delta_seconds();
            }
            if keys.pressed(KeyCode::KeyK) {
                player_right.x += MOVESPEED * time.delta_seconds();
            }
        }
    }

    // middle Point between players
    let player_middle_x = (player_left.x + player_right.x) / 2.0;
    let player_middle_y = (player_left.y + player_right.y) / 2.0;
    for mut transform in &mut query_middle {
        transform.translation.x = player_middle_x;
        transform.translation.y = player_middle_y;
    }

    // distance between players
    let distance = ((player_left.x - player_right.x).powi(2)
        + (player_left.y - player_right.y).powi(2))
    .sqrt();

    // limit line length
    if distance > LINE_LENGTH {
        // calculate new position for player_left and player_right
        let angle = (player_left.y - player_right.y).atan2(player_left.x - player_right.x);
        player_left.x = player_middle_x + LINE_LENGTH / 2.0 * angle.cos();
        player_left.y = player_middle_y + LINE_LENGTH / 2.0 * angle.sin();
        player_right.x = player_middle_x - LINE_LENGTH / 2.0 * angle.cos();
        player_right.y = player_middle_y - LINE_LENGTH / 2.0 * angle.sin();
    }
    // check player overlap
    if distance < PLAYER_WIDTH {
        let angle = (player_left.y - player_right.y).atan2(player_left.x - player_right.x);
        player_left.x = player_middle_x + (PLAYER_WIDTH / 2.0 + BONK_DISTANCE) * angle.cos();
        player_left.y = player_middle_y + (PLAYER_WIDTH / 2.0 + BONK_DISTANCE) * angle.sin();
        player_right.x = player_middle_x - (PLAYER_WIDTH / 2.0 + BONK_DISTANCE) * angle.cos();
        player_right.y = player_middle_y - (PLAYER_WIDTH / 2.0 + BONK_DISTANCE) * angle.sin();
    }

    // set new value
    for (player, mut transform) in &mut query {
        if player.left_hand {
            if player_left.x < SCREEN_WIDTH / 2.0 * -1.0 {
                player_left.x = SCREEN_WIDTH / 2.0 * -1.0;
            } else if player_left.x > SCREEN_WIDTH / 2.0 {
                player_left.x = SCREEN_WIDTH / 2.0;
            }
            if player_left.y < SCREEN_HEIGHT / 2.0 * -1.0 {
                player_left.y = SCREEN_HEIGHT / 2.0 * -1.0;
            } else if player_left.y > SCREEN_HEIGHT / 2.0 {
                player_left.y = SCREEN_HEIGHT / 2.0;
            }
            transform.translation.x = player_left.x;
            transform.translation.y = player_left.y;
        } else {
            if player_right.x < SCREEN_WIDTH / 2.0 * -1.0 {
                player_right.x = SCREEN_WIDTH / 2.0 * -1.0;
            } else if player_right.x > SCREEN_WIDTH / 2.0 {
                player_right.x = SCREEN_WIDTH / 2.0;
            }
            if player_right.y < SCREEN_HEIGHT / 2.0 * -1.0 {
                player_right.y = SCREEN_HEIGHT / 2.0 * -1.0;
            } else if player_right.y > SCREEN_HEIGHT / 2.0 {
                player_right.y = SCREEN_HEIGHT / 2.0;
            }
            transform.translation.x = player_right.x;
            transform.translation.y = player_right.y;
        }
    }
}

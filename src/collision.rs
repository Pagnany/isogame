use bevy::prelude::*;

use crate::enemy;
use crate::player;

pub fn player_with_enemy_over(
    query_player: Query<
        (&player::Player, &Transform),
        (With<player::Player>, Without<enemy::Enemy>),
    >,
    query_enemy: Query<(&enemy::Enemy, &Transform), (With<enemy::Enemy>, Without<player::Player>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for (_player, player_transform) in query_player.iter() {
        for (enemy, enemy_transform) in query_enemy.iter() {
            if enemy.under_water == enemy::EnemyType::AboveWater {
                // TODO 50 hardcoded!
                if player_transform.translation.x < enemy_transform.translation.x + 50.0
                    && player_transform.translation.x > enemy_transform.translation.x - 50.0
                    && player_transform.translation.y < enemy_transform.translation.y + 50.0
                    && player_transform.translation.y > enemy_transform.translation.y - 50.0
                {
                    next_state.set(crate::GameState::GameOver);
                }
            }
        }
    }
}

pub fn middle_with_enemy_under(
    query_enemy: Query<(&enemy::Enemy, &Transform), (With<enemy::Enemy>, Without<player::Player>)>,
    query_middle: Query<&Transform, (With<player::PlayerMiddle>, Without<player::Player>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for middle_transform in query_middle.iter() {
        for (enemy, enemy_transform) in query_enemy.iter() {
            if enemy.under_water == enemy::EnemyType::UnderWater {
                // TODO 50 hardcoded!
                if middle_transform.translation.x < enemy_transform.translation.x + 50.0
                    && middle_transform.translation.x > enemy_transform.translation.x - 50.0
                    && middle_transform.translation.y < enemy_transform.translation.y + 50.0
                    && middle_transform.translation.y > enemy_transform.translation.y - 50.0
                {
                    next_state.set(crate::GameState::GameOver);
                }
            }
        }
    }
}

pub fn player_with_player(
    mut query_player: Query<(&mut player::Player, &Transform), With<player::Player>>,
) {
    let mut player_left = player::PlayerPos { x: 0.0, y: 0.0 };
    let mut player_right = player::PlayerPos { x: 0.0, y: 0.0 };

    for (player, transform) in query_player.iter() {
        if player.left_hand {
            player_left.x = transform.translation.x;
            player_left.y = transform.translation.y;
        } else {
            player_right.x = transform.translation.x;
            player_right.y = transform.translation.y;
        }
    }
    if player_left.x < player_right.x + player::PLAYER_WIDTH
        && player_left.x > player_right.x - player::PLAYER_WIDTH
        && player_left.y < player_right.y + player::PLAYER_HIGHT
        && player_left.y > player_right.y - player::PLAYER_HIGHT
    {
        let angle1 = (player_left.y - player_right.y).atan2(player_left.x - player_right.x);
        let angle2 = (player_right.y - player_left.y).atan2(player_right.x - player_left.x);
        // add velocity to the player opposite to the angle
        query_player.iter_mut().for_each(|(mut player, _)| {
            if player.left_hand {
                player.velocity.x += angle1.cos() * player::BONK_DISTANCE;
                player.velocity.y += angle1.sin() * player::BONK_DISTANCE;
            } else {
                player.velocity.x += angle2.cos() * player::BONK_DISTANCE;
                player.velocity.y += angle2.sin() * player::BONK_DISTANCE;
            }
        });
    }
}

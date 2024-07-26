use bevy::prelude::*;

use crate::enemy;
use crate::player;

pub fn collision_player_enemy_over(
    query_player: Query<
        (&player::Player, &Transform),
        (With<player::Player>, Without<enemy::Enemy>),
    >,
    query_enemy: Query<(&enemy::Enemy, &Transform), (With<enemy::Enemy>, Without<player::Player>)>,
) {
    for (player, player_transform) in query_player.iter() {
        for (enemy, enemy_transform) in query_enemy.iter() {
            if enemy.under_water == enemy::EnemyType::AboveWater {
                // TODO 50 hardcoded!
                if player_transform.translation.x < enemy_transform.translation.x + 50.0
                    && player_transform.translation.x > enemy_transform.translation.x - 50.0
                    && player_transform.translation.y < enemy_transform.translation.y + 50.0
                    && player_transform.translation.y > enemy_transform.translation.y - 50.0
                {
                    println!("collision player enemy over");
                }
            }
        }
    }
}

pub fn collision_middle_enemy_under(
    query_enemy: Query<(&enemy::Enemy, &Transform), (With<enemy::Enemy>, Without<player::Player>)>,
    query_middle: Query<&Transform, (With<player::PlayerMiddle>, Without<player::Player>)>,
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
                    println!("collision middle enemy under");
                }
            }
        }
    }
}

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
    //println!("collision_player_enemy_over");
}

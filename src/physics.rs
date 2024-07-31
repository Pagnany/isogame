use crate::player::Player;
use bevy::prelude::*;

pub const DRAG: f32 = 100.0;

pub fn player_velocity_system(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        transform.translation.x += player.velocity.x * time.delta_seconds();
        if transform.translation.x < -1.0 * crate::SCREEN_WIDTH / 2.0 {
            transform.translation.x = -1.0 * crate::SCREEN_WIDTH / 2.0;
        } else if transform.translation.x > crate::SCREEN_WIDTH / 2.0 {
            transform.translation.x = crate::SCREEN_WIDTH / 2.0;
        }
        transform.translation.y += player.velocity.y * time.delta_seconds();
        if transform.translation.y < -1.0 * crate::SCREEN_HEIGHT / 2.0 {
            transform.translation.y = -1.0 * crate::SCREEN_HEIGHT / 2.0;
        } else if transform.translation.y > crate::SCREEN_HEIGHT / 2.0 {
            transform.translation.y = crate::SCREEN_HEIGHT / 2.0;
        }

        if player.velocity.x > 0.0 {
            player.velocity.x -= DRAG * time.delta_seconds();
            if player.velocity.x < 0.0 {
                player.velocity.x = 0.0;
            }
        } else if player.velocity.x < 0.0 {
            player.velocity.x += DRAG * time.delta_seconds();
            if player.velocity.x > 0.0 {
                player.velocity.x = 0.0;
            }
        }

        if player.velocity.y > 0.0 {
            player.velocity.y -= DRAG * time.delta_seconds();
            if player.velocity.y < 0.0 {
                player.velocity.y = 0.0;
            }
        } else if player.velocity.y < 0.0 {
            player.velocity.y += DRAG * time.delta_seconds();
            if player.velocity.y > 0.0 {
                player.velocity.y = 0.0;
            }
        }
    }
}

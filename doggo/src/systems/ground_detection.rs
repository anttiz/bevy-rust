use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{player_sprite::PlayerSprite};

pub fn ground_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &Transform)>,
    ground_query: Query<(&Collider, &Transform)>,
) {
    // Handle PlayerSprite ground detection
    for (mut player_sprite, transform) in sprite_query.iter_mut() {
        // player_sprite.on_ground = false; // Reset on_ground status
        let mut i = 0;
        // Check for ground collision
        for (ground_collider, ground_transform) in ground_query.iter() {
            let player_bottom_y = transform.translation.y; // Adjust based on player size
            let ground_top_y = ground_transform.translation.y + 0.5; // Adjust based on ground collider size
            if player_bottom_y > ground_top_y {
                // println!("PlayerSprite is on the ground {}, {}", player_bottom_y, ground_top_y);
                player_sprite.on_ground = true; // PlayerSprite is on the ground
                break; // No need to check further
            }
            i += 1;
        }
    }
}

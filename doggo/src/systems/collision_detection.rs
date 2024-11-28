use crate::game::constants::{SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH, STONE_HEIGHT};
use crate::game::{player_sprite::PlayerSprite, world::Stone};
use crate::systems::movement::restart_level;
use bevy::prelude::*;

pub fn collision_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &mut Transform)>,
    stone_query: Query<(&Stone, &Transform), Without<PlayerSprite>>,
) {
    // Handle PlayerSprite ground detection
    for (mut player_sprite, mut transform) in sprite_query.iter_mut() {
        // implement stone collision detection using stone_entities
        let player_x = transform.translation.x;
        let player_y = transform.translation.y;
        for (stone, stone_transform) in stone_query.iter() {
            // how to check if stone is stone and it collides with player_sprite?
            let stone_x = stone_transform.translation.x;
            let stone_y = stone_transform.translation.y;
            // check if collider is stone
            let stone_lower_y = stone_y + 44.0;
            let stone_upper_y = stone_lower_y + STONE_HEIGHT;
            let stone_right_x = stone_x + SPRITE_TILE_WIDTH;
            let stone_left_x = stone_x;
            if player_x + SPRITE_TILE_WIDTH > stone_left_x
                && player_x < stone_right_x
                && player_y > stone_lower_y
                && player_y < stone_upper_y
            {
                println!("Player collided with stone: {}, {}", player_x, stone_x);
                restart_level(&mut transform);
            }
        }
    }
}

use crate::game::constants::{SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH, STONE_HEIGHT};
use crate::game::{player_sprite::PlayerSprite, stone::Stone};
use crate::systems::movement::restart_level;
use bevy::prelude::*;

pub fn collision_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &mut Transform)>,
    stone_query: Query<(&Stone, &Transform), Without<PlayerSprite>>,
) {
    // Handle PlayerSprite ground detection
    for (mut player_sprite, mut transform) in sprite_query.iter_mut() {
        check_stone_collision(&mut transform, &stone_query);
    }
}

// New function to check for collision
fn is_colliding_with_stone(player_x: f32, player_y: f32, stone_x: f32, stone_y: f32) -> bool {
    let stone_lower_y = stone_y + 44.0;
    let stone_upper_y = stone_lower_y + STONE_HEIGHT;
    let stone_right_x = stone_x + SPRITE_TILE_WIDTH;
    let stone_left_x = stone_x;

    player_x + SPRITE_TILE_WIDTH > stone_left_x
        && player_x < stone_right_x
        && player_y > stone_lower_y
        && player_y < stone_upper_y
}

// Updated check_stone_collision function
fn check_stone_collision(
    player_transform: &mut Transform,
    stone_query: &Query<(&Stone, &Transform), Without<PlayerSprite>>,
) {
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    for (stone, stone_transform) in stone_query.iter() {
        let stone_x = stone_transform.translation.x;
        let stone_y = stone_transform.translation.y;

        if is_colliding_with_stone(player_x, player_y, stone_x, stone_y) {
            println!("Player collided with stone: {}, {}", player_x, stone_x);
            restart_level(player_transform);
        }
    }
}

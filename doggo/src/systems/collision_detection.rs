use crate::game::constants::{SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH, STONE_HEIGHT};
use crate::game::{player_sprite::PlayerSprite, deadly_item::DeadlyItem};
use crate::systems::movement::restart_level;
use bevy::prelude::*;

pub fn collision_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &mut Transform)>,
    deadly_item_query: Query<(&DeadlyItem, &Transform), Without<PlayerSprite>>,
) {
    // Handle PlayerSprite ground detection
    for (mut player_sprite, mut transform) in sprite_query.iter_mut() {
        check_deadly_item_collision(&mut transform, &deadly_item_query);
    }
}

// New function to check for collision
fn is_colliding_with_deadly_item(player_x: f32, player_y: f32, deadly_item_x: f32, deadly_item_y: f32) -> bool {
    let deadly_item_lower_y = deadly_item_y + 44.0;
    let deadly_item_upper_y = deadly_item_lower_y + STONE_HEIGHT;
    let deadly_item_right_x = deadly_item_x + SPRITE_TILE_WIDTH;
    let deadly_item_left_x = deadly_item_x;

    player_x + SPRITE_TILE_WIDTH > deadly_item_left_x
        && player_x < deadly_item_right_x
        && player_y > deadly_item_lower_y
        && player_y < deadly_item_upper_y
}

fn check_deadly_item_collision(
    player_transform: &mut Transform,
    deadly_item_query: &Query<(&DeadlyItem, &Transform), Without<PlayerSprite>>,
) {
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    for (deadly_item, deadly_item_transform) in deadly_item_query.iter() {
        let deadly_item_x = deadly_item_transform.translation.x;
        let deadly_item_y = deadly_item_transform.translation.y;

        if is_colliding_with_deadly_item(player_x, player_y, deadly_item_x, deadly_item_y) {
            println!("Player collided with item: {}, {}", player_x, deadly_item_x);
            restart_level(player_transform);
        }
    }
}

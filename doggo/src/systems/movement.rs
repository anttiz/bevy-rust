use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    constants::{
        GRAVITY_REDUCED, JUMP_VELOCITY, LEFT_BOUNDARY, PLAYER_VELOCITY_X, RIGHT_BOUNDARY,
        SPRITESHEET_COLS,
    },
    player::Player,
    player_sprite::{self, AnimationConfig, PlayerSprite},
};

pub fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player, &Transform)>,
    mut sprite_query: Query<
        (
            &mut KinematicCharacterController,
            &mut PlayerSprite,
            &Transform,
        ),
        Without<Player>,
    >,
) {
    let animation_config_walking_left: AnimationConfig = AnimationConfig::new(
        1 + SPRITESHEET_COLS as usize,
        3 + SPRITESHEET_COLS as usize,
        10,
    );
    let animation_config_walking_right: AnimationConfig = AnimationConfig::new(
        1 + SPRITESHEET_COLS as usize * 2,
        3 + SPRITESHEET_COLS as usize * 2,
        10,
    );
    let animation_config_standing = AnimationConfig::new(1, 3, 10);

    // Handle Player movement
    for (mut player_controller, mut player, transform) in query.iter_mut() {
        let mut translation = Vec2::new(0.0, 0.0);

        if !player.on_ground {
            player.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
        } else {
            player.vertical_velocity = 0.0; // Reset vertical velocity when on ground
        }

        if input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
        }

        if input.just_pressed(KeyCode::Space) && player.on_ground {
            player.vertical_velocity += JUMP_VELOCITY;
            player.on_ground = false;
        }

        translation.y += player.vertical_velocity * time.delta_seconds();

        // Boundary checks for PlayerSprite
        let new_position =
            player_controller.translation.unwrap_or_default() + transform.translation.xy();
        // println!("New Position: {}, Left Boundary: {}", new_position.x, LEFT_BOUNDARY);
        // prevent player from moving left of the left boundary
        if new_position.x < LEFT_BOUNDARY {
            translation.x = LEFT_BOUNDARY - new_position.x;
        }
        // prevent player from moving right of the right boundary
        if new_position.x > RIGHT_BOUNDARY {
            translation.x = RIGHT_BOUNDARY - new_position.x;
        }

        player_controller.translation = Some(translation);
    }

    // Handle PlayerSprite movement
    for (mut sprite_controller, mut player_sprite, transform) in sprite_query.iter_mut() {
        let mut translation = Vec2::new(0.0, 0.0);

        if !player_sprite.on_ground {
            player_sprite.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
        } else {
            player_sprite.vertical_velocity = 0.0; // Reset vertical velocity when on ground
        }

        if input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;

            // Update the animation index for walking right
            player_sprite.current_animation_index =
                animation_config_walking_right.first_sprite_index;
        } else if player_sprite.on_ground {
            // Reset to standing animation if on ground and no movement
            player_sprite.current_animation_index = animation_config_standing.first_sprite_index;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;

            // Update the animation index for walking left
            player_sprite.current_animation_index =
                animation_config_walking_left.first_sprite_index;
        }

        if input.just_pressed(KeyCode::Space) && player_sprite.on_ground {
            player_sprite.vertical_velocity += JUMP_VELOCITY;
            player_sprite.on_ground = false;
        }

        translation.y += player_sprite.vertical_velocity * time.delta_seconds();
        // Boundary checks for PlayerSprite
        let new_position =
            sprite_controller.translation.unwrap_or_default() + transform.translation.xy();
        // println!("New Position: {}, Left Boundary: {}", new_position.x, LEFT_BOUNDARY);
        // prevent player from moving left of the left boundary
        if new_position.x < LEFT_BOUNDARY {
            translation.x = LEFT_BOUNDARY - new_position.x;
        }
        // prevent player from moving right of the right boundary
        if new_position.x > RIGHT_BOUNDARY {
            translation.x = RIGHT_BOUNDARY - new_position.x;
        }

        sprite_controller.translation = Some(translation);
    }
}

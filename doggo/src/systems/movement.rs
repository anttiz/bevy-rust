use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    constants::{
        COLLISION_THRESHOLD, GRAVITY_REDUCED, JUMP_VELOCITY, LEFT_BOUNDARY, PLAYER_START_X,
        PLAYER_START_Y, PLAYER_VELOCITY_X, RIGHT_BOUNDARY, STONE_WIDTH,
    },
    elevator::Elevator,
    level_config::{get_current_level, set_current_level, LEVELS},
    player::Player,
    player_sprite::{enter_next_level, PlayerSprite},
    stone::Stone,
    world::{respawn_world, SkyBarEntities, StoneEntities},
    CurrentLevel,
};

pub fn movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut Player,
        &mut Transform,
    )>,
    mut sprite_query: Query<
        (
            &mut KinematicCharacterController,
            &mut PlayerSprite,
            &mut Transform,
        ),
        Without<Player>,
    >,
    mut stone_entities: ResMut<StoneEntities>,
    mut sky_bar_entities: ResMut<SkyBarEntities>,
    mut current_level: ResMut<CurrentLevel>,
) {
    // Handle Player movement
    for (mut player_controller, mut player, mut transform) in query.iter_mut() {
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

        // Reset rotation to prevent turning
        transform.rotation = Quat::from_rotation_z(0.0);
    }

    // Handle PlayerSprite movement
    for (mut sprite_controller, mut player_sprite, mut transform) in sprite_query.iter_mut() {
        let mut translation = Vec2::new(0.0, 0.0);

        if player_sprite.health == 0 {
            restart_level(&mut transform);
            respawn_world(commands, stone_entities, sky_bar_entities);
            player_sprite.health = 100;
            return;
        }

        if !player_sprite.on_ground {
            player_sprite.vertical_velocity += GRAVITY_REDUCED * time.delta_seconds();
        } else {
            player_sprite.vertical_velocity = 0.0; // Reset vertical velocity when on ground
        }

        if input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            translation.x += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
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
            // enter next level
            enter_next_level(
                commands,
                sprite_controller,
                transform,
                stone_entities,
                sky_bar_entities,
                current_level,
            );
            return;
        }

        sprite_controller.translation = Some(translation);

        // Reset rotation to prevent turning
        transform.rotation = Quat::from_rotation_z(0.0);
    }
}

pub fn restart_level(player_transform: &mut Transform) {
    player_transform.translation = Vec3::new(PLAYER_START_X, PLAYER_START_Y, 1.0);
}

pub fn move_stones(time: Res<Time>, mut stone_query: Query<(&Stone, &mut Transform)>) {
    for (stone, mut transform) in stone_query.iter_mut() {
        if stone.speed > 0.0 {
            // Only move if speed is greater than zero
            transform.translation.x += stone.direction.x * stone.speed * time.delta_seconds();

            // Check if the stone has moved past the left boundary
            if transform.translation.x < LEFT_BOUNDARY {
                // Move the stone to the right side of the screen
                transform.translation.x = RIGHT_BOUNDARY + STONE_WIDTH; // Adjust as needed
            }
        }
    }
}

pub fn move_elevators(time: Res<Time>, mut query: Query<(&mut Transform, &mut Elevator)>) {
    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    let speed = level_config.elevator_speed;
    for (mut transform, mut elevator) in query.iter_mut() {
        let direction = if elevator.moving_up { 1.0 } else { -1.0 };
        transform.translation.y += direction * speed * time.delta_seconds();

        // Change direction when reaching certain heights
        if transform.translation.y > level_config.elevator_end_y {
            // Upper limit
            elevator.moving_up = false;
        } else if transform.translation.y < level_config.elevator_start_y {
            // Lower limit
            elevator.moving_up = true;
        }
    }
}

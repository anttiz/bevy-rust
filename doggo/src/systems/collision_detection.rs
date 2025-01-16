use crate::{game::{deadly_item::DeadlyItem, player_sprite::PlayerSprite}, systems::movement::restart_level};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionEvent};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct OnGround;

pub fn collision_detection(
    mut sprite_query: Query<(&mut PlayerSprite, &mut Transform)>,
    deadly_item_query: Query<(&DeadlyItem, &Transform), Without<PlayerSprite>>,
) {
    for (mut player_sprite, mut transform) in sprite_query.iter_mut() {
        check_deadly_item_collision(&mut transform, &deadly_item_query, &mut player_sprite);
    }
}

// New function to check for collision
fn is_colliding_with_deadly_item(player_x: f32, player_y: f32,
    deadly_item_center_x: f32, deadly_item_center_y: f32,
    deadly_item_width: f32, deadly_item_height: f32) -> bool {
    if deadly_item_width == 0.0 || deadly_item_height == 0.0 {
        return false;
    }
    let player_height = 128.0;
    let player_lower_y = player_y - player_height / 2.0;
    let player_upper_y = player_lower_y + player_height;
    let player_width = 90.0;
    let player_left_x = player_x - player_width / 2.0;
    let player_right_x = player_left_x + player_width;

    let deadly_item_lower_y = deadly_item_center_y - deadly_item_height / 2.0;
    let deadly_item_upper_y = deadly_item_center_y + deadly_item_height / 2.0;
    let deadly_item_right_x = deadly_item_center_x + deadly_item_width / 2.0;
    let deadly_item_left_x = deadly_item_center_x - deadly_item_width / 2.0;

    // println!("Player bounding box: {}, {}, {}, {}", player_left_x, player_right_x, player_lower_y, player_upper_y);
    //println!("Deadly item bounding box: {}, {}, {}, {}", deadly_item_left_x, deadly_item_right_x, deadly_item_lower_y, deadly_item_upper_y);
    player_right_x > deadly_item_left_x
        && player_left_x < deadly_item_right_x
        && player_upper_y > deadly_item_lower_y
        && player_lower_y < deadly_item_upper_y
}

fn check_deadly_item_collision(
    player_transform: &mut Transform,
    deadly_item_query: &Query<(&DeadlyItem, &Transform), Without<PlayerSprite>>,
    player_sprite: &mut PlayerSprite,
) {
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    for (deadly_item, deadly_item_transform) in deadly_item_query.iter() {
        let deadly_item_center_x = deadly_item_transform.translation.x;
        let deadly_item_center_y = deadly_item_transform.translation.y;
        if is_colliding_with_deadly_item(player_x, player_y, deadly_item_center_x, deadly_item_center_y,
            deadly_item.width, deadly_item.height) {
            println!("Player collided with item: {}, {}", player_x, deadly_item_center_x);
            println!("Player y: {}, Deadly item y: {}", player_y, deadly_item_center_y);
            player_sprite.health = 0;
            restart_level(player_transform);
        }
    }
}

pub fn collision_detection_with_collider(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &PlayerSprite)>,
    collider_query: Query<&Collider>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if one of the entities is the horse
                if let Ok((horse_entity, _)) = player_query.get(*entity1) {
                    // Check if the other entity is a ground collider
                    if collider_query.get(*entity2).is_ok() {
                        // Handle the collision (e.g., set on_ground to true)
                        println!("Player collided with ground");
                        commands.entity(horse_entity).insert(OnGround);
                    }
                } else if let Ok((horse_entity, _)) = player_query.get(*entity2) {
                    // Check if the other entity is a ground collider
                    if collider_query.get(*entity1).is_ok() {
                        // Handle the collision (e.g., set on_ground to true)
                        commands.entity(horse_entity).insert(OnGround);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                // Check if one of the entities is the horse
                if let Ok((horse_entity, _)) = player_query.get(*entity1) {
                    // Check if the other entity is a ground collider
                    if collider_query.get(*entity2).is_ok() {
                        // Handle the collision end (e.g., set on_ground to false)
                        commands.entity(horse_entity).remove::<OnGround>();
                    }
                } else if let Ok((horse_entity, _)) = player_query.get(*entity2) {
                    // Check if the other entity is a ground collider
                    if collider_query.get(*entity1).is_ok() {
                        // Handle the collision end (e.g., set on_ground to false)
                        commands.entity(horse_entity).remove::<OnGround>();
                    }
                }
            }
        }
    }
}
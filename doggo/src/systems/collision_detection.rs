use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::{player::Player, player_sprite::PlayerSprite};
use crate::game::world::StoneEntities;

/*
pub fn collision_detection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &PlayerSprite)>,
    stone_entities: Res<StoneEntities>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("CollisionEvent::Started: {:?} {:?}", entity1, entity2);
                // Check if the player is involved in the collision
                if let Ok((player_entity, _)) = player_query.get(*entity1) {
                    // Check if entity2 is a stone
                    if stone_entities.0.contains(entity2) {
                        println!("Player collided with stone_entities: {:?}", entity2);
                        // Additional logic here
                    }
                } else if let Ok((player_entity, _)) = player_query.get(*entity2) {
                    // Check if entity1 is a stone
                    if stone_entities.0.contains(entity1) {
                        println!("Player collided with stone_entities: {:?}", entity1);
                        // Additional logic here
                    }
                }
            }
            _ => {}
        }
    }
}
*/

pub fn collision_detection(
  mut collision_events: EventReader<CollisionEvent>,
  player_query: Query<Entity, With<PlayerSprite>>,
  stone_entities: Res<StoneEntities>,
) {
    for event in collision_events.read() {
        println!("CollisionEvent: {:?}", event);
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
              let player_entity = player_query.single();
              if *entity1 == player_entity || *entity2 == player_entity {
                  let stone_entity = if *entity1 == player_entity { *entity2 } else { *entity1 };
                  if stone_entities.0.contains(&stone_entity) {
                      println!("Player collided with stone: {:?}", stone_entity);
                      // Add your collision response logic here
                  }
              }
          }
          CollisionEvent::Stopped(_, _, _) => {
              // Handle collision end if needed
          }
      }
  }
}
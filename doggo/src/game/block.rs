use crate::game::constants::*;
use crate::game::level_config::LevelConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::world::BlockEntities;

#[derive(Component)]
pub struct Block;

pub fn spawn_block(commands: &mut Commands, position: Vec3, width: f32, height: f32) -> Entity {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BLOCK_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: position,
                scale: Vec3::new(width, height, BLOCK_Z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5)) // Set collider
        .insert(Block)
        .id();
    return entity;
}

pub fn spawn_blocks(commands: &mut Commands, level_config: &LevelConfig, block_entities: &mut ResMut<BlockEntities>) {
    for block in &level_config.blocks {
        let entity = spawn_block(
            commands,
            Vec3::new(block.start_x, block.start_y, 1.0),
            block.width,
            block.height,
        );
        block_entities.0.push(entity);
    }
}

use bevy::prelude::*;
use crate::game::constants::*;
use crate::game::level_config::LevelConfig;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Block;

pub fn spawn_block(commands: &mut Commands, position: Vec3, width: f32, height: f32) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 1.0), // Block color
                ..Default::default()
            },
            transform: Transform {
                translation: position,
                scale: Vec3::new(width, height, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(width / 2.0, height / 2.0)) // Set collider
        .insert(Block);
}

pub fn spawn_blocks(commands: &mut Commands, level_config: &LevelConfig) {
    for block in &level_config.blocks {
        spawn_block(
            commands,
            Vec3::new(block.start_x, block.start_y, 0.0),
            block.width,
            block.height,
        );
    }
}

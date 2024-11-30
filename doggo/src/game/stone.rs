use bevy::prelude::*;
use crate::game::constants::*;
use crate::game::level_config::LevelConfig;
use crate::game::world::StoneEntities;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Stone;

impl Default for Stone {
    fn default() -> Self {
        Stone {}
    }
}

pub fn spawn_stones(
    commands: &mut Commands,
    level_config: &LevelConfig,
    stone_entities: &mut ResMut<StoneEntities>,
) {
    // Calculate the starting position to center the stones
    let total_width = (level_config.stone_count as f32 - 1.0) * level_config.stone_interval;
    let start_x_pos = -total_width / 2.0;

    for i in 0..level_config.stone_count {
        let x_pos = start_x_pos + i as f32 * level_config.stone_interval; // Adjusted position
        let entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x_pos, GRASS_TOP_Y + 90.0, 0.0),
                    scale: Vec3::new(STONE_WIDTH, STONE_HEIGHT, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                STONE_CUBOID_WIDTH / 2.0,
                STONE_CUBOID_HEIGHT / 2.0,
            ))
            .insert(Stone::default())
            .id(); // Get the entity ID

        stone_entities.0.push(entity); // Store the entity ID
    }
}

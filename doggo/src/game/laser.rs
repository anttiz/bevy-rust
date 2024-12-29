use crate::game::constants::*;
use crate::game::level_config::LevelConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::deadly_item::DeadlyItem;
use super::world::LaserEntities;

#[derive(Component)]
pub struct Laser {
    pub length: f32,
    pub laser_on_time_ms: f32,
    pub laser_off_time_ms: f32,
    pub start_x: f32,
}

impl Default for Laser {
    fn default() -> Self {
        Laser {
            length: 0.0,
            laser_on_time_ms: 0.0,
            laser_off_time_ms: 0.0,
            start_x: 0.0,
        }
    }
}

pub fn spawn_lasers(
    commands: &mut Commands,
    level_config: &LevelConfig,
    laser_entities: &mut ResMut<LaserEntities>,
) {
    for i in 0..level_config.lasers.len() {
        let x_pos = level_config.lasers[i].start_x;
        let length = level_config.lasers[i].length;
        let entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.0, 1.0, 1.0), // Pink color
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x_pos, LASER_HEIGHT_FROM_GROUND, 0.0),
                    scale: Vec3::new(length, LASER_WIDTH, 1.0), // Define LASER_WIDTH and LASER_HEIGHT
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                LASER_CUBOID_WIDTH / 2.0,
                LASER_CUBOID_HEIGHT / 2.0,
            ))
            .insert(DeadlyItem {
                width: length,
                height: LASER_WIDTH,
            })
            .insert(Laser {
                length: level_config.lasers[i].length,
                laser_on_time_ms: level_config.lasers[i].laser_on_time_ms,
                laser_off_time_ms: level_config.lasers[i].laser_off_time_ms,
                start_x: level_config.lasers[i].start_x,
            })
            .id(); // Get the entity ID

        laser_entities.0.push(entity); // Store the entity ID

        // Debugging output to confirm laser creation
        // println!("Spawned laser at x: {}", x_pos);
    }
}

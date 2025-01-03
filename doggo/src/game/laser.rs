use std::time::Duration;

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
    pub timer: Timer,
    pub is_visible: bool,
}

impl Default for Laser {
    fn default() -> Self {
        Laser {
            length: 0.0,
            laser_on_time_ms: 1000.0,
            laser_off_time_ms: 1000.0,
            start_x: 0.0,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            is_visible: true,
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
                    color: Color::srgba(1.0, 0.0, 1.0, 1.0),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x_pos, LASER_HEIGHT_FROM_GROUND, 0.0),
                    scale: Vec3::new(length, LASER_WIDTH, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(DeadlyItem {
                width: length,
                height: LASER_WIDTH,
            })
            .insert(Laser {
                length: level_config.lasers[i].length,
                laser_on_time_ms: level_config.lasers[i].laser_on_time_ms,
                laser_off_time_ms: level_config.lasers[i].laser_off_time_ms,
                start_x: level_config.lasers[i].start_x,
                timer: Timer::from_seconds(2.0, TimerMode::Once),
                is_visible: true,
            })
            .id();

        laser_entities.0.push(entity);
    }
}

pub fn update_laser_visibility(
    time: Res<Time>,
    mut query: Query<(&mut Laser, &mut Visibility, &mut DeadlyItem)>,
) {
    for (mut laser, mut visibility, mut deadly_item) in query.iter_mut() {
        laser.timer.tick(time.delta());

        if laser.timer.finished() {
            laser.is_visible = !laser.is_visible;
            let new_duration = if laser.is_visible {
                laser.laser_on_time_ms
            } else {
                laser.laser_off_time_ms
            } as u64;
            *visibility = if laser.is_visible { Visibility::Inherited } else { Visibility::Hidden };

            // Update DeadlyItem based on visibility
            if laser.is_visible {
                deadly_item.width = laser.length; // Set to deadly dimensions
                deadly_item.height = LASER_WIDTH;
            } else {
                deadly_item.width = 0.0; // Set to non-deadly dimensions
                deadly_item.height = 0.0;
            }

            laser.timer = Timer::from_seconds(new_duration as f32 / 1000.0, TimerMode::Once);
        }
    }
}

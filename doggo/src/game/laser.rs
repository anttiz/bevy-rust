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
    pub timer: Timer,
    pub is_visible: bool,
}

impl Default for Laser {
    fn default() -> Self {
        Laser {
            length: 0.0,
            laser_on_time_ms: 1000.0,
            laser_off_time_ms: 1000.0,
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
    for laser in &level_config.lasers {
        let entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: LASER_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(laser.start_x, LASER_HEIGHT_FROM_GROUND, LASER_Z),
                    scale: Vec3::new(laser.length, LASER_WIDTH, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(DeadlyItem {
                width: laser.length,
                height: LASER_WIDTH,
            })
            .insert(Laser {
                length: laser.length,
                laser_on_time_ms: laser.laser_on_time_ms,
                laser_off_time_ms: laser.laser_off_time_ms,
                timer: Timer::from_seconds(LASER_INITIAL_TIME_SECS, TimerMode::Once),
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

            if laser.is_visible {
                deadly_item.width = laser.length;
                deadly_item.height = LASER_WIDTH;
            } else {
                deadly_item.width = 0.0;
                deadly_item.height = 0.0;
            }

            laser.timer = Timer::from_seconds(new_duration as f32 / 1000.0, TimerMode::Once);
        }
    }
}

use super::constants::*;
use super::level_config::LEVELS;
use super::level_config::{get_current_level, set_current_level};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_world(mut commands: Commands) {
    // Print level names
    for level in LEVELS {
        println!("Level Name: {}", level.name);
    }
    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    // Sky
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_SKY,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_HEIGHT * 0.5, 0.0),
            scale: Vec3::new(
                WINDOW_WIDTH,
                WINDOW_HEIGHT * SKY_HEIGHT_PERCENT / 100.0 * 2.0,
                1.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    // Grass
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_GRASS,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, GRASS_TOP_Y, 0.0),
            scale: Vec3::new(
                -WINDOW_WIDTH,
                WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0,
                1.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    // Calculate the starting position to center the stones
    let total_width = (level_config.stone_count as f32 - 1.0) * level_config.stone_interval;
    let start_x_pos = -total_width / 2.0;

    for i in 0..level_config.stone_count {
        let x_pos = start_x_pos + i as f32 * level_config.stone_interval; // Adjusted position
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x_pos, GRASS_TOP_Y + 90.0, 0.0),
                    scale: Vec3::new(40.0, 40.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5));
    }
    // Floor
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_TOP_Y + 70.0, 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}

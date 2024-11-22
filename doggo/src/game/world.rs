use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::constants::*;
use super::level_config::LEVELS;

pub fn spawn_world(mut commands: Commands) {
    // Print level names
    for level in LEVELS {
        println!("Level Name: {}", level.name);
    }

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

    // Stone platform (set to fixed)
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_TOP_Y + 90.0, 0.0),
                scale: Vec3::new(40.0, 40.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed) // Set to Fixed to make it static
        .insert(Collider::cuboid(0.5, 0.5)); // Adjust the size of the collider as needed

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

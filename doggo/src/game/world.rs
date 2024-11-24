use super::constants::*;
use super::level_config::LEVELS;
use super::level_config::{get_current_level, set_current_level, LevelConfig};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct StoneEntities(pub Vec<Entity>);

pub fn spawn_world(mut commands: Commands, mut stone_entities: ResMut<StoneEntities>) {
    // Despawn previous stones
    for entity in stone_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    stone_entities.0.clear(); // Clear the vector for the new level

    // Print level names
    for level in LEVELS {
        println!("Level Name: {}", level.name);
    }
    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    spawn_sky(&mut commands);
    spawn_stones(&mut commands, level_config, &mut stone_entities);
    spawn_grass(&mut commands);
    spawn_floor(&mut commands);
}

// New function to spawn grass
fn spawn_grass(commands: &mut Commands) {
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
}

fn spawn_floor(commands: &mut Commands) {
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

fn spawn_stones(commands: &mut Commands, level_config: &LevelConfig, stone_entities: &mut ResMut<StoneEntities>) {
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
                    scale: Vec3::new(40.0, 40.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.5, 0.5))
            .id(); // Get the entity ID

        stone_entities.0.push(entity); // Store the entity ID
    }
}

fn spawn_sky(commands: &mut Commands) {
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
}

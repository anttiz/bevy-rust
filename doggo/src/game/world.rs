use super::constants::*;
use super::level_config::LEVELS;
use super::level_config::{get_current_level, set_current_level, LevelConfig};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::stone::spawn_stones;

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

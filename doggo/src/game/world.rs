use super::constants::*;
use super::level_config::LEVELS;
use super::level_config::{get_current_level};
use crate::game::elevator::spawn_elevators;
use crate::game::grass::spawn_grass;
use crate::game::sky_bar::spawn_sky_bars;
use crate::game::stone::spawn_stones;
use crate::game::block::spawn_blocks;
use crate::game::sky::spawn_sky;
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Resource)]
pub struct StoneEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct SkyBarEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct ElevatorEntities(pub Vec<Entity>);

pub fn spawn_world(
    mut commands: Commands,
    mut stone_entities: ResMut<StoneEntities>,
    mut sky_bar_entities: ResMut<SkyBarEntities>,
    mut elevator_entities: ResMut<ElevatorEntities>,
) {
    despawn_previous_entities(&mut commands, &mut stone_entities, &mut sky_bar_entities);

    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    spawn_sky(&mut commands);
    spawn_stones(&mut commands, level_config, &mut stone_entities);
    spawn_grass(&mut commands);
    spawn_floor(&mut commands);
    spawn_sky_bars(&mut commands, level_config, &mut sky_bar_entities);
    spawn_blocks(&mut commands, level_config);
    spawn_elevators(&mut commands, level_config, &mut elevator_entities);
}

pub fn despawn_previous_entities(
    commands: &mut Commands,
    stone_entities: &mut ResMut<StoneEntities>,
    sky_bar_entities: &mut ResMut<SkyBarEntities>,
) {
    // Despawn previous stones
    for entity in stone_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    stone_entities.0.clear(); // Clear the vector for the new level

    // Despawn previous sky bars
    for entity in sky_bar_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    sky_bar_entities.0.clear(); // Clear the vector for the new level
}

pub fn respawn_world(
    mut commands: Commands,
    mut stone_entities: ResMut<StoneEntities>,
    mut sky_bar_entities: ResMut<SkyBarEntities>,
    mut elevator_entities: ResMut<ElevatorEntities>,
) {
    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    despawn_previous_entities(&mut commands, &mut stone_entities, &mut sky_bar_entities);
    spawn_stones(&mut commands, level_config, &mut stone_entities);
    spawn_sky_bars(&mut commands, level_config, &mut sky_bar_entities);
    spawn_elevators(&mut commands, level_config, &mut elevator_entities);
}

fn spawn_floor(commands: &mut Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, GRASS_CENTER_Y + 70.0, 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}

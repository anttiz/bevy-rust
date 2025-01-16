use super::level_config::LEVELS;
use super::level_config::get_current_level;
use crate::game::elevator::spawn_elevators;
use crate::game::grass::spawn_grass;
use crate::game::sky_bar::spawn_sky_bars;
use crate::game::stone::spawn_stones;
use crate::game::block::spawn_blocks;
use crate::game::sky::spawn_sky;
use crate::game::sun::spawn_sun;
use crate::game::laser::spawn_lasers;
use bevy::ecs::system::Resource;
use bevy::prelude::*;
#[derive(Resource)]
pub struct StoneEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct SkyBarEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct ElevatorEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct LaserEntities(pub Vec<Entity>);
#[derive(Resource)]
pub struct BlockEntities(pub Vec<Entity>);

pub fn spawn_world(
    commands: &mut Commands,
    mut stone_entities: ResMut<StoneEntities>,
    mut sky_bar_entities: ResMut<SkyBarEntities>,
    mut elevator_entities: ResMut<ElevatorEntities>,
    mut laser_entities: ResMut<LaserEntities>,
    mut block_entities: ResMut<BlockEntities>,
    asset_server: Res<AssetServer>,
) {
    despawn_previous_entities(commands, &mut stone_entities,
        &mut sky_bar_entities, &mut laser_entities, &mut elevator_entities, &mut block_entities);

    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    spawn_sky(commands, &asset_server);
    spawn_stones(commands, level_config, &mut stone_entities);
    spawn_grass(commands);
    spawn_sky_bars(commands, level_config, &mut sky_bar_entities);
    spawn_blocks(commands, level_config, &mut block_entities);
    spawn_elevators(commands, level_config, &mut elevator_entities);
    spawn_lasers(commands, level_config, &mut laser_entities);
}

pub fn despawn_previous_entities(
    commands: &mut Commands,
    stone_entities: &mut ResMut<StoneEntities>,
    sky_bar_entities: &mut ResMut<SkyBarEntities>,
    laser_entities: &mut ResMut<LaserEntities>,
    elevator_entities: &mut ResMut<ElevatorEntities>,
    block_entities: &mut ResMut<BlockEntities>,
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

    // Despawn previous lasers
    for entity in laser_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    laser_entities.0.clear(); // Clear the vector for the new level

    // Despawn previous elevators
    for entity in elevator_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    elevator_entities.0.clear(); // Clear the vector for the new level

    // Despawn previous blocks
    for entity in block_entities.0.iter() {
        commands.entity(*entity).despawn();
    }
    block_entities.0.clear(); // Clear the vector for the new level
}

pub fn respawn_world(
    commands: &mut Commands,
    mut stone_entities: ResMut<StoneEntities>,
    mut sky_bar_entities: ResMut<SkyBarEntities>,
    mut elevator_entities: ResMut<ElevatorEntities>,
    mut laser_entities: ResMut<LaserEntities>,
    mut block_entities: ResMut<BlockEntities>,
) {
    let current_level = get_current_level();
    let level_config = &LEVELS[current_level];
    despawn_previous_entities(commands, &mut stone_entities,
        &mut sky_bar_entities, &mut laser_entities, &mut elevator_entities, &mut block_entities);
    spawn_stones(commands, level_config, &mut stone_entities);
    spawn_sky_bars(commands, level_config, &mut sky_bar_entities);
    spawn_elevators(commands, level_config, &mut elevator_entities);
    spawn_lasers(commands, level_config, &mut laser_entities);
}

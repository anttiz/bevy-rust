pub mod constants;
pub mod level_config;
pub mod player_sprite;
pub mod world;
pub mod stone;
pub mod sky_bar;
pub mod deadly_item;
pub mod elevator;
pub mod grass;
pub mod block;
pub mod sky;
pub mod cloud;
pub mod sun;
pub mod laser;
use bevy::prelude::*;
use world::{BlockEntities, ElevatorEntities, LaserEntities, SkyBarEntities, StoneEntities};
use crate::systems::audio::insert_audio_resource;
use self::{player_sprite::spawn_player as spawn_player_sprite, world::spawn_world};


#[derive(Resource)]
pub struct CurrentLevel {
    pub level: usize,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        CurrentLevel { level: 0 }
    }
}


#[derive(Resource)]
struct DyingSound(Handle<AudioSource>);

// for meshes and materials
pub fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    stone_entities: ResMut<StoneEntities>,
    sky_bar_entities: ResMut<SkyBarEntities>,
    elevator_entities: ResMut<ElevatorEntities>,
    laser_entities: ResMut<LaserEntities>,
    block_entities: ResMut<BlockEntities>,
) {
    commands.spawn(Camera2dBundle::default());

    insert_audio_resource(&mut commands, &asset_server);
    spawn_player_sprite(&mut commands, &asset_server, texture_atlas_layouts);
    spawn_world(&mut commands, stone_entities, sky_bar_entities, elevator_entities, laser_entities, block_entities, asset_server);
}

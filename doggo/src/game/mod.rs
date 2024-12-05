pub mod constants;
pub mod level_config;
pub mod player;
pub mod player_sprite;
pub mod world;
pub mod stone;
pub mod sky_bar;
pub mod deadly_item;
pub mod elevator;
pub mod grass;
pub mod block;
use bevy::prelude::*;
use world::{StoneEntities, SkyBarEntities};



use self::{player_sprite::spawn_player as spawn_player_sprite, world::spawn_world};

// for meshes and materials
pub fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    stone_entities: ResMut<StoneEntities>,
    sky_bar_entities: ResMut<SkyBarEntities>,
) {
    commands.spawn(Camera2dBundle::default());
    // spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_player_sprite(&mut commands, asset_server, texture_atlas_layouts);
    spawn_world(commands, stone_entities, sky_bar_entities);
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub level: usize,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        CurrentLevel { level: 0 }
    }
}

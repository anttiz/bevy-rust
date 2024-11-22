pub mod constants;
pub mod player;
pub mod player_sprite;
pub mod world;
pub mod level_config;

use bevy::prelude::*;

use self::{world::spawn_world, player_sprite::spawn_player as spawn_player_sprite};

// for meshes and materials
pub fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    // spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_player_sprite(&mut commands, asset_server, texture_atlas_layouts);
    spawn_world(commands);
}

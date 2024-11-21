pub mod constants;
pub mod player;
pub mod world;

use bevy::prelude::*;

use self::{player::spawn_player, world::spawn_world, player::spawn_player_sprite};

// for meshes and materials
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_player_sprite(&mut commands, asset_server, texture_atlas_layouts);
    spawn_world(commands);
}

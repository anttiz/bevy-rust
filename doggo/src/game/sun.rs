use crate::game::constants::*;
use bevy::prelude::*;

pub fn spawn_sun(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let sun_texture_handle = asset_server.load("textures/sun.png");

    commands.spawn(SpriteBundle {
        texture: sun_texture_handle.clone(),
        transform: Transform {
            translation: Vec3::new(SUN_X, SUN_Y, SUN_Z),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

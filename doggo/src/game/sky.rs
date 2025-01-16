use crate::game::cloud::spawn_cloud;
use crate::game::constants::*;
use crate::game::sun::spawn_sun;
use bevy::prelude::*;

pub fn spawn_sky(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_SKY,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, SKY_CENTER_Y, SKY_Z),
            scale: Vec3::new(WINDOW_WIDTH, SKY_HEIGHT, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    spawn_cloud(commands, &asset_server);
    spawn_sun(commands, &asset_server);
}

use crate::game::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_grass(commands: &mut Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_GRASS,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, GRASS_CENTER_Y, GRASS_Z),
            scale: Vec3::new(WINDOW_WIDTH, GRASS_HEIGHT, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5, 0.5));
}
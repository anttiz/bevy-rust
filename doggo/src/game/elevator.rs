use crate::{
    game::constants::{ELEVATOR_HEIGHT, ELEVATOR_WIDTH},
    ELEVATOR_COLOR,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
#[derive(Component)]
pub struct Elevator {
    pub moving_up: bool,
    pub index: usize,
}

impl Default for Elevator {
    fn default() -> Self {
        Elevator { moving_up: true, index: 0 }
    }
}

pub fn spawn_elevator(commands: &mut Commands, position: Vec3, index: usize) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: ELEVATOR_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: position,
                scale: Vec3::new(ELEVATOR_WIDTH, ELEVATOR_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Elevator {
            moving_up: true,
            index,
        });
}

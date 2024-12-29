use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct DeadlyItem {
    pub width: f32,
    pub height: f32,
}

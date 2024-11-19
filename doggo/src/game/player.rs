use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use super::constants::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 1.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default())
        .insert(Player);
}

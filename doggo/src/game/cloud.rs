use crate::game::constants::*;
use bevy::prelude::*;
use rand;
#[derive(Component)]
pub struct Cloud;

pub fn spawn_cloud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let cloud_texture_handle = asset_server.load("textures/cloud.png");
    let y = CLOUD_Y + rand::random::<f32>() * CLOUD_Y_VARIANCE;
    commands.spawn(SpriteBundle {
        texture: cloud_texture_handle.clone(),
        transform: Transform {
            translation: Vec3::new(CLOUD_X, y, CLOUD_Z),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Cloud);
}

pub fn animate_clouds(
    time: Res<Time>,
    mut query: Query<(&Cloud, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x -= CLOUD_SPEED * time.delta_seconds();

        if transform.translation.x < -800.0 {
            transform.translation.x = 800.0;
        }
    }
}

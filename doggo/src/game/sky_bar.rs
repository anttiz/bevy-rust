use crate::game::constants::*;
use crate::game::level_config::LevelConfig;
use crate::game::world::SkyBarEntities;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// Define the SkyBar component
#[derive(Component)]
pub struct SkyBar {
    // Add any necessary properties here
}

pub fn spawn_sky_bars(
    commands: &mut Commands,
    level_config: &LevelConfig,
    sky_bar_entities: &mut ResMut<SkyBarEntities>,
) {
    for sky_bar in &level_config.sky_bars {
        let x_pos = sky_bar.start_x;
        let entity = setup_sky_bar(commands, x_pos);
        sky_bar_entities.0.push(entity);
    }
}

// Implement the SkyBar logic
fn setup_sky_bar(commands: &mut Commands, x_pos: f32) -> Entity {
    const SKY_BAR_HEIGHT: f32 = WINDOW_TOP_Y - SKY_BAR_START_Y;
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SKY_BAR_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x_pos, SKY_BAR_START_Y + SKY_BAR_HEIGHT / 2.0, SKY_BAR_Z),
                scale: Vec3::new(SKY_BAR_WIDTH, SKY_BAR_HEIGHT, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            SKY_BAR_CUBOID_WIDTH / 2.0,
            SKY_BAR_CUBOID_HEIGHT / 2.0,
        ))
        .insert(SkyBar {
            // Initialize properties if needed
        })
        .id();
    entity
}

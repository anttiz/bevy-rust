use crate::game::constants::*;
// use crate::game::deadly_item::DeadlyItem;
use crate::game::level_config::LevelConfig;
use crate::game::player_sprite::PlayerSprite;
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
    // Calculate the starting position to center the stones
    let total_width = (level_config.sky_bars as f32 - 1.0) * level_config.sky_bar_interval;
    let start_x_pos = -total_width / 2.0;

    for i in 0..level_config.sky_bars {
        let x_pos = start_x_pos + i as f32 * level_config.sky_bar_interval; // Adjusted position
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
        // .insert(DeadlyItem)
        .insert(SkyBar {
        // Initialize properties if needed
    })
        .id();
    return entity;
}

// Collision detection logic
fn check_horse_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<PlayerSprite>>,
    sky_bar_query: Query<(Entity, &Transform), With<SkyBar>>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (sky_bar_entity, sky_bar_transform) in sky_bar_query.iter() {
            if player_transform.translation.y > sky_bar_transform.translation.y {
                // Logic for when the horse touches the Sky Bar
                commands.entity(player_entity).despawn(); // Example: despawn horse on collision
            }
        }
    }
}

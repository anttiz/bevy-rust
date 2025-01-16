use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod systems;

use game::constants::*;
use game::level_config::get_current_level;
use game::player_sprite::trigger_animation;
use game::player_sprite::execute_animations;
use bevy::input::common_conditions::input_just_pressed;
use game::setup;
use game::cloud::animate_clouds;
use game::world::{BlockEntities, LaserEntities, StoneEntities, SkyBarEntities, ElevatorEntities};
use game::CurrentLevel;
use systems::ground_detection::ground_detection;
use systems::movement::movement;
use systems::collision_detection::collision_detection;
use systems::collision_detection::collision_detection_with_collider;
use systems::movement::move_stones;
use systems::ui::{setup_level_ui, update_level_ui};
use systems::movement::move_elevators;
use game::laser::update_laser_visibility;
fn main() {
    App::new()
        .insert_resource(StoneEntities(Vec::new()))
        .insert_resource(SkyBarEntities(Vec::new()))
        .insert_resource(ElevatorEntities(Vec::new()))
        .insert_resource(LaserEntities(Vec::new()))
        .insert_resource(BlockEntities(Vec::new()))
        .insert_resource(CurrentLevel { level: get_current_level() })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Platformer".to_string(),
                        resizable: false,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_clouds)
        .add_systems(Update, collision_detection_with_collider)
        .add_systems(Update, collision_detection)
        .add_systems(Update, movement)
        .add_systems(Update, ground_detection)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            trigger_animation.run_if(
                input_just_pressed(KeyCode::ArrowRight)
                    .or_else(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .add_systems(Update, move_stones)
        .add_systems(Startup, setup_level_ui)
        .add_systems(Update, update_level_ui)
        .add_systems(Update, move_elevators)
        .add_systems(Update, update_laser_visibility)
        .run();
}

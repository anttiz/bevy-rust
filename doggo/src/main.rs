use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod systems;

use game::constants::*;
use game::player_sprite::trigger_animation;
use bevy::input::common_conditions::input_just_pressed;
use game::setup;
use systems::movement::execute_animations;
use systems::movement::movement;
use systems::ground_detection::ground_detection;
use crate::game::player_sprite::WalkingLeftSprite;
use crate::game::player_sprite::WalkingRightSprite;
fn main() {
    App::new()
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
        .add_systems(Update, movement)
        .add_systems(Update, ground_detection)
        .add_systems(Update, execute_animations)
        .add_systems(
            Update,
            (
                // press the right arrow key to animate the right sprite
                trigger_animation::<WalkingRightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                // press the left arrow key to animate the left sprite
                trigger_animation::<WalkingLeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .run();
}

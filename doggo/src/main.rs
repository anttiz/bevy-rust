use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod systems;

use game::constants::*;
use game::player_sprite::trigger_animation;
use game::player_sprite::execute_animations;
use bevy::input::common_conditions::input_just_pressed;
use game::setup;
use systems::ground_detection::ground_detection;
use systems::movement::movement;

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
            trigger_animation.run_if(
                input_just_pressed(KeyCode::ArrowRight)
                    .or_else(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        )
        .run();
}

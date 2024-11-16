use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_string(),
                resizable: false,
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Draw the sky
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_HEIGHT * 0.5, 0.0), // Centered vertically
            scale: Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT * 1.4, 1.0), // 70% height
            ..Default::default()
        },
        ..Default::default()
    });

    // Draw the grass
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (WINDOW_HEIGHT * 0.15), 0.0), // Position it at the bottom
            scale: Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT * 0.3, 1.0), // 30% height
            ..Default::default()
        },
        ..Default::default()
    });

    // Optionally, spawn other sprites (e.g., the rectangles you had before)
    /*
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.0),
            scale: Vec3::new(75.0, 200.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.0),
            scale: Vec3::new(50.0, 350.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.0),
            scale: Vec3::new(150.0, 250.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    */

    commands.spawn(Camera2dBundle::default());
}

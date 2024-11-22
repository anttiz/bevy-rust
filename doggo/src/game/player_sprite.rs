use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use super::constants::{
    ANIMATION_FPS, ANIMATION_FRAMES, SPRITESHEET_COLS, SPRITESHEET_ROWS, SPRITE_TILE_HEIGHT,
    SPRITE_TILE_WIDTH, STANDING_SPRITE_FIRST_INDEX, WALKING_LEFT_SPRITE_FIRST_INDEX,
    WALKING_RIGHT_SPRITE_FIRST_INDEX,
};

#[derive(Component)]
pub struct PlayerSprite {
    pub vertical_velocity: f32,
    pub on_ground: bool,
}

impl Default for PlayerSprite {
    fn default() -> Self {
        PlayerSprite {
            vertical_velocity: 0.0,
            on_ground: true,
        }
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Component)]
struct StandingSprite;

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/horses.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_TILE_WIDTH as u32, SPRITE_TILE_HEIGHT as u32),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        Some(UVec2::new(10, 10)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config_standing = AnimationConfig::new(
        STANDING_SPRITE_FIRST_INDEX,
        STANDING_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1,
        ANIMATION_FPS,
    );

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(1.0))
                    .with_translation(Vec3::new(-250.0, 0.0, 1.0)),
                texture: texture.clone(),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_standing.first_sprite_index,
            },
            StandingSprite,
            animation_config_standing,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(20.0, SPRITE_TILE_HEIGHT / 2.0))
        .insert(KinematicCharacterController::default())
        .insert(PlayerSprite::default());
}

pub fn trigger_animation(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut animation, mut texture_atlas) in query.iter_mut() {
        if input.pressed(KeyCode::ArrowRight) {
            texture_atlas.index = WALKING_RIGHT_SPRITE_FIRST_INDEX;
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        } else if input.pressed(KeyCode::ArrowLeft) {
            texture_atlas.index = WALKING_LEFT_SPRITE_FIRST_INDEX;
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        } else {
            texture_atlas.index = STANDING_SPRITE_FIRST_INDEX;
        }
    }
}

fn get_current_state(atlas: &TextureAtlas) -> String {
    if atlas.index >= STANDING_SPRITE_FIRST_INDEX
        && atlas.index <= STANDING_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1
    {
        return "standing".to_string();
    } else if atlas.index >= WALKING_LEFT_SPRITE_FIRST_INDEX
        && atlas.index <= WALKING_LEFT_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1
    {
        return "walking_left".to_string();
    } else if atlas.index >= WALKING_RIGHT_SPRITE_FIRST_INDEX
        && atlas.index <= WALKING_RIGHT_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1
    {
        return "walking_right".to_string();
    }
    "standing".to_string()
}

fn get_last_sprite_index(state: &str) -> usize {
    match state {
        "standing" => STANDING_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1,
        "walking_left" => WALKING_LEFT_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1,
        "walking_right" => WALKING_RIGHT_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1,
        _ => STANDING_SPRITE_FIRST_INDEX + ANIMATION_FRAMES - 1,
    }
}

fn get_first_sprite_index(state: &str) -> usize {
    get_last_sprite_index(state) + 1 - ANIMATION_FRAMES
}

pub fn execute_animations(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            let current_state = get_current_state(&atlas);
            let next_state = if input.pressed(KeyCode::ArrowRight) {
                "walking_right".to_string()
            } else if input.pressed(KeyCode::ArrowLeft) {
                "walking_left".to_string()
            } else {
                "standing".to_string()
            };

            if current_state != next_state {
                atlas.index = get_first_sprite_index(&next_state);
            }

            if atlas.index == get_last_sprite_index(&next_state) {
                atlas.index = get_first_sprite_index(&next_state);
            } else {
                atlas.index += 1;
            }

            if next_state != "standing" {
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

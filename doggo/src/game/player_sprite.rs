use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;
use bevy::input::ButtonInput;

use super::constants::{SPRITESHEET_COLS, SPRITESHEET_ROWS, SPRITE_TILE_HEIGHT, SPRITE_TILE_WIDTH};

#[derive(Component)]
pub struct PlayerSprite {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    pub current_animation_index: usize,
}

impl Default for PlayerSprite {
    fn default() -> Self {
        PlayerSprite {
            vertical_velocity: 0.0,
            on_ground: true,
            current_animation_index: 0,
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

#[derive(Component)]
pub struct WalkingLeftSprite;

#[derive(Component)]
pub struct WalkingRightSprite;

const STANDING_SPRITE_FIRST_INDEX: usize = 1;
const WALKING_LEFT_SPRITE_FIRST_INDEX: usize = 1 + SPRITESHEET_COLS as usize;
const WALKING_RIGHT_SPRITE_FIRST_INDEX: usize = 1 + SPRITESHEET_COLS as usize * 2;

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

    let animation_config_standing = AnimationConfig::new(1, 3, 10);
    let animation_config_walking_left: AnimationConfig = AnimationConfig::new(
        1 + SPRITESHEET_COLS as usize,
        3 + SPRITESHEET_COLS as usize,
        10,
    );
    let animation_config_walking_right: AnimationConfig = AnimationConfig::new(
        1 + SPRITESHEET_COLS as usize * 2,
        3 + SPRITESHEET_COLS as usize * 2,
        10,
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
            println!("Walking right");
            texture_atlas.index = WALKING_RIGHT_SPRITE_FIRST_INDEX;
            // animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        } else if input.pressed(KeyCode::ArrowLeft) {
            println!("Walking left");
            texture_atlas.index = WALKING_LEFT_SPRITE_FIRST_INDEX;
            //animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        } else {
            texture_atlas.index = STANDING_SPRITE_FIRST_INDEX;
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
  time: Res<Time>,
  mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
  for (mut config, mut atlas) in &mut query {
      // we track how long the current sprite has been displayed for
      config.frame_timer.tick(time.delta());

      // If it has been displayed for the user-defined amount of time (fps)...
      if config.frame_timer.just_finished() {
          if atlas.index == config.last_sprite_index {
              // ...and it IS the last frame, then we move back to the first frame and stop.
              atlas.index = config.first_sprite_index;
          } else {
              // ...and it is NOT the last frame, then we move to the next frame...
              atlas.index += 1;
              // ...and reset the frame timer to start counting all over again
              config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
          }
      }
  }
}

use bevy::prelude::*;

// Window constants
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_TOP_Y: f32 = WINDOW_HEIGHT / 2.0;

// Game world constants
pub const SKY_HEIGHT_PERCENT: f32 = 80.0;
// Grass constants
pub const GRASS_HEIGHT_PERCENT: f32 = 100.0 - SKY_HEIGHT_PERCENT;
pub const GRASS_HEIGHT: f32 = WINDOW_HEIGHT * GRASS_HEIGHT_PERCENT / 100.0;
pub const GRASS_CENTER_Y: f32 = WINDOW_BOTTOM_Y + GRASS_HEIGHT / 2.0;

// Sky constants
pub const SKY_HEIGHT: f32 = WINDOW_HEIGHT * SKY_HEIGHT_PERCENT / 100.0;
pub const SKY_CENTER_Y: f32 = WINDOW_TOP_Y - SKY_HEIGHT / 2.0;

// Sun constants
pub const SUN_Y: f32 = WINDOW_HEIGHT / 2.0 - 100.0;
pub const SUN_X: f32 = WINDOW_WIDTH / 2.0 - 100.0;
pub const SUN_Z:f32 = SKY_Z;

// Cloud constants
pub const CLOUD_Y: f32 = WINDOW_HEIGHT / 2.0 - 160.0;
pub const CLOUD_X: f32 = WINDOW_WIDTH / 2.0 - 160.0;
pub const CLOUD_Z: f32 = SUN_Z + 0.5;
pub const CLOUD_SPEED: f32 = 100.0;
pub const CLOUD_Y_VARIANCE: f32 = 100.0;

// Laser constants
pub const LASER_WIDTH: f32 = 5.0;
pub const LASER_HEIGHT_FROM_GROUND: f32 = WINDOW_BOTTOM_Y + GRASS_HEIGHT + 50.0;
pub const LASER_INITIAL_TIME_SECS: f32 = 0.5;

// Floor constants
pub const PLAYER_VELOCITY_X: f32 = 400.0;
pub const JUMP_VELOCITY: f32 = 30000.0;
pub const GRAVITY_REDUCTION_FACTOR: f32 = 0.1;
pub const GRAVITY: f32 = -9.81;
pub const GRAVITY_REDUCED: f32 = GRAVITY * GRAVITY_REDUCTION_FACTOR;
pub const LEFT_BOUNDARY: f32 = -WINDOW_WIDTH / 2.0 + PLAYER_WIDTH;
pub const RIGHT_BOUNDARY: f32 = WINDOW_WIDTH / 2.0 - PLAYER_WIDTH;

// Player constants
pub const PLAYER_START_X: f32 = LEFT_BOUNDARY + 1.0;
pub const PLAYER_START_Y: f32 = 0.0; // Starting Y position of the player
pub const PLAYER_WIDTH: f32 = 30.0; // Width of the player
pub const PLAYER_SCALE: f32 = 2.0; // Scale of the player
// Colors
// Platform - Dark Brown
pub const COLOR_PLATFORM: Color = Color::srgba(139.0 / 255.0, 69.0 / 255.0, 19.0 / 255.0, 255.0); // SaddleBrown
// Sky - Light Blue
pub const COLOR_SKY: Color = Color::srgba(135.0 / 255.0, 206.0 / 255.0, 235.0 / 255.0, 255.0); // LightSkyBlue
// Grass - Bright Green
pub const COLOR_GRASS: Color = Color::srgba(0.0, 255.0, 127.0 / 255.0, 255.0); // SpringGreen

// grass constants
pub const GRASS_Z: f32 = 0.0;

pub const STONE_WIDTH: f32 = 30.0;
pub const STONE_HEIGHT: f32 = 30.0;
pub const STONE_CUBOID_WIDTH: f32 = 1.0;
pub const STONE_CUBOID_HEIGHT: f32 = 1.0;
pub const STONE_SPEED: f32 = 100.0;
pub const STONE_Z: f32 = 1.0;
pub const SKY_BAR_WIDTH: f32 = 30.0;
pub const SKY_BAR_START_Y: f32 = 0.0;
pub const SKY_BAR_CUBOID_WIDTH: f32 = 1.0;
pub const SKY_BAR_CUBOID_HEIGHT: f32 = 1.0;
// Sky Bar - Light Tan
pub const SKY_BAR_COLOR: Color = Color::srgba(222.0 / 255.0, 184.0 / 255.0, 135.0 / 255.0, 255.0); // Burlywood
pub const SKY_BAR_Z: f32 = 1.0;

// Elevator constants
pub const ELEVATOR_HEIGHT: f32 = 30.0;
pub const ELEVATOR_WIDTH: f32 = 4.0 * ELEVATOR_HEIGHT;
// Elevator - Light Brown
pub const ELEVATOR_COLOR: Color = Color::srgba(210.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0, 255.0); // Tan
pub const ELEVATOR_Z: f32 = 1.0;

// Spritesheet constants
pub const SPRITE_TILE_WIDTH: f32 = 64.0;
pub const SPRITE_TILE_HEIGHT: f32 = 64.0;
pub const SPRITESHEET_COLS: u32 = 12;
pub const SPRITESHEET_ROWS: u32 = 8;

// block constants
pub const BLOCK_Z: f32 = 1.0;
// Block - Dark Gray
pub const BLOCK_COLOR: Color = Color::srgba(169.0 / 255.0, 169.0 / 255.0, 169.0 / 255.0, 255.0); // DarkGray

// laser constants
pub const LASER_Z: f32 = 1.0;
// Laser - Bright Red
pub const LASER_COLOR: Color = Color::srgba(255.0, 0.0, 0.0, 255.0); // Red

// sky constants
pub const SKY_Z: f32 = 0.0;

// animation constants
pub const ANIMATION_FPS: u8 = 10;
pub const ANIMATION_FRAMES: usize = 3;
pub const STANDING_SPRITE_FIRST_INDEX: usize = 0;
pub const WALKING_LEFT_SPRITE_FIRST_INDEX: usize =
    STANDING_SPRITE_FIRST_INDEX + SPRITESHEET_COLS as usize;
pub const WALKING_RIGHT_SPRITE_FIRST_INDEX: usize =
    STANDING_SPRITE_FIRST_INDEX + SPRITESHEET_COLS as usize * 2;

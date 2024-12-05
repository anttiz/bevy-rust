use crate::game::constants::STONE_SPEED;
use std::sync::Mutex;

// Add this at the top of the file to import the lazy_static crate
use lazy_static::lazy_static; // Import lazy_static

const STARTING_LEVEL: usize = 7;
// Define a global variable for the current level
lazy_static! {
    pub static ref CURRENT_LEVEL: Mutex<usize> = Mutex::new(STARTING_LEVEL); // Initialize to level 0
}

use super::constants::GRASS_TOP_Y;

pub struct LevelConfig {
    pub index: usize,          // Index of the level
    pub stone_count: usize,    // Amount of stones in the level
    pub stones_moving: bool,   // Boolean if stones are moving
    pub stone_interval: f32,   // Stone interval in pixels
    pub stone_speed: f32,      // Stone speed in pixels per second (optional)
    pub sky_bars: usize,       // Number of sky bars in the level
    pub sky_bar_interval: f32, // Sky bar interval in pixels
    pub elevator_count: usize, // Number of elevators in the level
    pub elevator_start_x: f32, // Elevator start x position
    pub elevator_start_y: f32, // Elevator start y position
    pub elevator_speed: f32,    // Elevator speed in pixels per second
    pub elevator_end_y: f32,    // Elevator end y position
}

pub const LEVELS: &[LevelConfig] = &[
    LevelConfig {
        index: 0,
        stone_count: 1,
        stones_moving: false,
        stone_interval: 50.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 1,
        stone_count: 2,
        stones_moving: false,
        stone_interval: 400.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 2,
        stone_count: 2,
        stones_moving: false,
        stone_interval: 350.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 3,
        stone_count: 1,
        stones_moving: true,
        stone_interval: 350.0,
        stone_speed: 2.0 * STONE_SPEED,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 4,
        stone_count: 2,
        stones_moving: true,
        stone_interval: 300.0,
        stone_speed: 2.0 * STONE_SPEED,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 5,
        stone_count: 0,
        stones_moving: false,
        stone_interval: 0.0,
        stone_speed: 0.0,
        sky_bars: 2,
        sky_bar_interval: 250.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 6,
        stone_count: 1,
        stones_moving: true,
        stone_interval: 300.0,
        stone_speed: STONE_SPEED,
        sky_bars: 2,
        sky_bar_interval: 300.0,
        elevator_count: 0,
        elevator_start_x: 0.0,
        elevator_start_y: 0.0,
        elevator_speed: 0.0,
        elevator_end_y: 0.0,
    },
    LevelConfig {
        index: 7,
        stone_count: 0,
        stones_moving: false,
        stone_interval: 0.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
        elevator_count: 1,
        elevator_start_x: 100.0,
        elevator_start_y: GRASS_TOP_Y + 50.0,
        elevator_speed: 50.0,
        elevator_end_y: GRASS_TOP_Y + 200.0,

    },
];

// Example function to set the current level
pub fn set_current_level(level_index: usize) {
    let mut current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    if level_index >= LEVELS.len() {
        // println!("Level index out of bounds");
        return;
    }
    *current_level = level_index; // Set the current level
}

// Example function to get the current level
pub fn get_current_level() -> usize {
    let current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    *current_level // Return the current level
}

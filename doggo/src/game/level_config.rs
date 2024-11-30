use crate::game::constants::STONE_SPEED;
use std::sync::Mutex;

// Add this at the top of the file to import the lazy_static crate
use lazy_static::lazy_static; // Import lazy_static

const STARTING_LEVEL: usize = 6;
// Define a global variable for the current level
lazy_static! {
    pub static ref CURRENT_LEVEL: Mutex<usize> = Mutex::new(STARTING_LEVEL); // Initialize to level 0
}

pub struct LevelConfig {
    pub name: &'static str,  // Name of the level as a string slice
    pub index: usize,        // Index of the level
    pub stone_count: usize,  // Amount of stones in the level
    pub stones_moving: bool, // Boolean if stones are moving
    pub stone_interval: f32, // Stone interval in pixels
    pub stone_speed: f32,    // Stone speed in pixels per second (optional)
    pub sky_bars: usize,     // Number of sky bars in the level
    pub sky_bar_interval: f32, // Sky bar interval in pixels
}

pub const LEVELS: &[LevelConfig] = &[
    LevelConfig {
        name: "Level 1",
        index: 0,
        stone_count: 1,
        stones_moving: false,
        stone_interval: 50.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
    },
    LevelConfig {
        name: "Level 2",
        index: 1,
        stone_count: 2,
        stones_moving: false,
        stone_interval: 400.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
    },
    LevelConfig {
        name: "Level 3",
        index: 2,
        stone_count: 2,
        stones_moving: false,
        stone_interval: 350.0,
        stone_speed: 0.0,
        sky_bars: 0,
        sky_bar_interval: 0.0,
    },
    LevelConfig {
        name: "Level 4",
        index: 3,
        stone_count: 1,
        stones_moving: true,
        stone_interval: 350.0,
        stone_speed: 2.0 * STONE_SPEED,
        sky_bars: 0,
        sky_bar_interval: 0.0,
    },
    LevelConfig {
        name: "Level 5",
        index: 4,
        stone_count: 2,
        stones_moving: true,
        stone_interval: 300.0,
        stone_speed: 2.0 * STONE_SPEED,
        sky_bars: 0,
        sky_bar_interval: 0.0,
    },
    LevelConfig {
        name: "Level 6",
        index: 5,
        stone_count: 0,
        stones_moving: false,
        stone_interval: 0.0,
        stone_speed: 0.0,
        sky_bars: 2,
        sky_bar_interval: 250.0,
    },
    LevelConfig {
        name: "Level 7",
        index: 6,
        stone_count: 1,
        stones_moving: true,
        stone_interval: 300.0,
        stone_speed: STONE_SPEED,
        sky_bars: 2,
        sky_bar_interval: 300.0,
    },
    ];

// Example function to set the current level
pub fn set_current_level(level_index: usize) {
    let mut current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    if level_index >= LEVELS.len() {
        println!("Level index out of bounds");
        return;
    }
    *current_level = level_index; // Set the current level
}

// Example function to get the current level
pub fn get_current_level() -> usize {
    let current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    *current_level // Return the current level
}

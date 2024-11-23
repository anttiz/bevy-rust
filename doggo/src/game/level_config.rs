use std::sync::Mutex;

// Add this at the top of the file to import the lazy_static crate
use lazy_static::lazy_static; // Import lazy_static

// Define a global variable for the current level
lazy_static! {
    pub static ref CURRENT_LEVEL: Mutex<usize> = Mutex::new(0); // Initialize to level 0
}

pub struct LevelConfig {
    pub name: &'static str,         // Name of the level as a string slice
    pub index: usize,                // Index of the level
    pub stone_count: usize,          // Amount of stones in the level
    pub stones_moving: bool,         // Boolean if stones are moving
    pub stone_interval: f32,          // Stone interval in pixels
}

pub const LEVELS: &[LevelConfig] = &[
    LevelConfig {
        name: "Level 1",         // Use a string literal
        index: 0,
        stone_count: 10,         // Amount of stones
        stones_moving: true,     // Stones are moving
        stone_interval: 50.0,    // Stone interval in pixels
    },
    // Add more levels as needed
];

// Example function to set the current level
pub fn set_current_level(level_index: usize) {
    let mut current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    *current_level = level_index; // Set the current level
}

// Example function to get the current level
pub fn get_current_level() -> usize {
    let current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    *current_level // Return the current level
}

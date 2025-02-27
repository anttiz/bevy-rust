use crate::{game::constants::STONE_SPEED, ELEVATOR_WIDTH};
use lazy_static::lazy_static;
use std::sync::Mutex; // Import lazy_static
use serde::{Deserialize, Serialize}; // Add serde for JSON serialization/deserialization
use std::fs::File; // Import File for file handling
use std::io::Read; // Import Read for reading file content

const STARTING_LEVEL: usize = 0;
const STARTING_ATTEMPT_COUNT: usize = 0;
use super::constants::GRASS_CENTER_Y;

// Define a struct for Elevator properties
#[derive(Clone, Serialize, Deserialize)]
pub struct ElevatorConfig {
    pub start_x: f32, // Elevator start x position
    pub start_y: f32, // Elevator start y position
    pub speed: f32,   // Elevator speed in pixels per second
    pub end_y: f32,   // Elevator end y position
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BlockConfig {
    pub start_x: f32, // Block start x position
    pub start_y: f32, // Block start y position
    pub width: f32,    // Block width
    pub height: f32,   // Block height
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LaserConfig {
    pub start_x: f32, // Laser start x position
    pub length: f32,   // Laser length
    pub laser_on_time_ms: f32, // Laser on time in milliseconds
    pub laser_off_time_ms: f32, // Laser off time in milliseconds
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SkyBarConfig {
    pub start_x: f32, // Sky bar start x position
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub stone_count: usize,             // Amount of stones in the level
    pub stones_moving: bool,            // Boolean if stones are moving
    pub stone_interval: f32,            // Stone interval in pixels
    pub stone_speed: f32,               // Stone speed in pixels per second (optional)
    pub sky_bars: Vec<SkyBarConfig>,    // Array of sky bars
    pub elevators: Vec<ElevatorConfig>, // Array of elevators
    pub blocks: Vec<BlockConfig>,       // Array of blocks
    pub lasers: Vec<LaserConfig>,       // Array of lasers
}

// Function to load level configurations from a JSON file
pub fn load_level_configs_from_json(file_path: &str) -> Vec<LevelConfig> {
    let mut file = File::open(file_path).expect("Unable to open file"); // Open the JSON file
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file"); // Read the file content
    serde_json::from_str(&contents).expect("JSON was not well-formatted") // Deserialize JSON to Vec<LevelConfig>
}

// Function to get the current level configurations
pub fn get_level_configs() -> Vec<LevelConfig> {
    // Uncomment the line below to load from JSON
    // load_level_configs_from_json("path/to/your/levels.json")

    // Keep the hardcoded levels as a fallback
    vec![
        LevelConfig {
            stone_count: 1,
            stones_moving: false,
            stone_interval: 50.0,
            stone_speed: 0.0,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 2,
            stones_moving: false,
            stone_interval: 400.0,
            stone_speed: 0.0,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 2,
            stones_moving: false,
            stone_interval: 350.0,
            stone_speed: 0.0,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 1,
            stones_moving: true,
            stone_interval: 400.0,
            stone_speed: 2.0 * STONE_SPEED,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 2,
            stones_moving: true,
            stone_interval: 400.0,
            stone_speed: 2.0 * STONE_SPEED,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 0,
            stones_moving: false,
            stone_interval: 0.0,
            stone_speed: 0.0,
            sky_bars: vec![
                SkyBarConfig {
                    start_x: 100.0,
                },
                SkyBarConfig {
                    start_x: 200.0,
                },
            ],
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 2,
            stones_moving: true,
            stone_interval: 300.0,
            stone_speed: STONE_SPEED,
            sky_bars: vec![
                SkyBarConfig {
                    start_x: -100.0,
                },
                SkyBarConfig {
                    start_x: 100.0,
                },
            ],
            elevators: vec![], // Initialize as empty vector
            blocks: vec![], // Initialize as empty vector
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 0,
            stones_moving: false,
            stone_interval: 0.0,
            stone_speed: 0.0,
            sky_bars: vec![], // Initialize as empty vector
            elevators: vec![
                ElevatorConfig {
                    start_x: 100.0,
                    start_y: GRASS_CENTER_Y + 50.0,
                    speed: 50.0,
                    end_y: GRASS_CENTER_Y + 300.0,
                },
                ElevatorConfig {
                    start_x: -150.0,
                    start_y: GRASS_CENTER_Y + 50.0,
                    speed: 80.0,
                    end_y: GRASS_CENTER_Y + 300.0,
                },
                ],
            blocks: vec![
                BlockConfig {
                    start_x: 100.0 + ELEVATOR_WIDTH,
                    start_y: GRASS_CENTER_Y + 100.0,
                    width: 100.0,
                    height: 550.0,
                },
            ],
            lasers: vec![], // Initialize as empty vector
        },
        LevelConfig {
            stone_count: 0,
            stones_moving: false,
            stone_interval: 0.0,
            stone_speed: 0.0,
            sky_bars: vec![],
            elevators: vec![],
            blocks: vec![],
            lasers: vec![LaserConfig {
                start_x: 100.0,
                length: 500.0,
                laser_on_time_ms: 100.0,
                laser_off_time_ms: 3000.0,
            }],
        },
        LevelConfig {
            stone_count: 1,
            stones_moving: true,
            stone_interval: 500.0,
            stone_speed: 1.0 * STONE_SPEED,
            sky_bars: vec![],
            elevators: vec![],
            blocks: vec![],
            lasers: vec![LaserConfig {
                start_x: 100.0,
                length: 500.0,
                laser_on_time_ms: 100.0,
                laser_off_time_ms: 1000.0,
            }],
        },
        LevelConfig {
            stone_count: 1,
            stones_moving: true,
            stone_interval: 300.0,
            stone_speed: 1.0 * STONE_SPEED,
            sky_bars: vec![
                SkyBarConfig {
                    start_x: 100.0,
                },
                SkyBarConfig {
                    start_x: 300.0,
                },
            ],
            elevators: vec![
                ElevatorConfig {
                    start_x: -100.0,
                    start_y: GRASS_CENTER_Y + 50.0,
                    speed: 60.0,
                    end_y: GRASS_CENTER_Y + 200.0,
                },
            ],
            blocks: vec![],
            lasers: vec![
                LaserConfig {
                    start_x: -100.0,
                    length: 400.0,
                    laser_on_time_ms: 200.0,
                    laser_off_time_ms: 3000.0,
                },
            ],
        },
        ]
}

// Define a global variable for the current level
lazy_static! {
    pub static ref CURRENT_LEVEL: Mutex<usize> = Mutex::new(STARTING_LEVEL); // Initialize to level 0

    // Change LEVELS to a static reference
    pub static ref LEVELS: Vec<LevelConfig> = get_level_configs(); // Load from either JSON or hardcoded values

    pub static ref ATTEMPT_COUNT: Mutex<usize> = Mutex::new(STARTING_ATTEMPT_COUNT); // Initialize to level 0
}

// Example function to set the current level
pub fn set_current_level(level_index: usize) {
    let mut current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    if level_index >= LEVELS.len() {
        return;
    }
    *current_level = level_index; // Set the current level
    let mut attempt_count = ATTEMPT_COUNT.lock().unwrap(); // Access ATTEMPT_COUNT directly
    *attempt_count = STARTING_ATTEMPT_COUNT; // Reset the attempt count
}

// Example function to get the current level
pub fn get_current_level() -> usize {
    let current_level = CURRENT_LEVEL.lock().unwrap(); // Access CURRENT_LEVEL directly
    *current_level // Return the current level
}

pub fn get_attempt_count() -> usize {
    let attempt_count = ATTEMPT_COUNT.lock().unwrap(); // Access ATTEMPT_COUNT directly
    *attempt_count // Return the attempt count
}

pub fn increase_attempt_count() {
    let mut attempt_count = ATTEMPT_COUNT.lock().unwrap(); // Access ATTEMPT_COUNT directly
    *attempt_count += 1; // Increment the attempt count
}
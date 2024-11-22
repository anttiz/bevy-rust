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

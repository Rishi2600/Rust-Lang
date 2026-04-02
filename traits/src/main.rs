struct GameConfig {
    resolution: (u32, u32),
    difficulty: String,
    fullscreen: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            difficulty: "Normal".to_string(),
            fullscreen: true,
        }
    }
}

fn main() {
    // Superpower: "Give me the default, but change the difficulty"
    let hard_mode = GameConfig {
        difficulty: "Hard".to_string(),
        ..Default::default() // The "Splat" operator works with traits!
    };
}
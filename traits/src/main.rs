#[derive(Debug)] // This trait lets us print the struct with {:?}
struct GameConfig {
    resolution: (u32, u32),
    difficulty: String,
    fullscreen: bool,
}

// Implementing the 'Default' trait
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
    // 1. Create a totally default config
    let basic_config = GameConfig::default();
    
    // 2. The "Splat" (Struct Update Syntax)
    // We only change the difficulty; the rest is copied from Default
    let hard_mode = GameConfig {
        difficulty: "Hard".to_string(),
        ..Default::default() 
    };

    println!("Standard: {:?}", basic_config);
    println!("Hard Mode: {:?}", hard_mode);
}
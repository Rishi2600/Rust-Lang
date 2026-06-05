use std::sync::OnceLock;

// Global, thread-safe, lazily-initialized variable
static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static str {
    // The closure inside 'get_or_init' only runs the first time this function is called
    GLOBAL_CONFIG.get_or_init(|| {
        println!("--- Loading configuration from disk (EXPENSIVE) ---");
        String::from("DATABASE_URL=localhost; PORT=5432;")
    })
}

fn main() {
    println!("Main started. No config loaded yet.");
    
    // Call 1: Runs the initialization logic
    let config1 = get_config();
    
    // Call 2: Returns a direct reference immediately without running the closure
    let config2 = get_config();

    println!("Config 1: {}", config1);
    println!("Config 2: {}", config2);
}
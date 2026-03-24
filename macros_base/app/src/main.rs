use my_macros::AutoCli;

#[derive(AutoCli)]
struct Config {
    name: String,
    port: u16,
}

fn main() {
    // Run this using: cargo run -p app -- --name Rishi --port 3000
    let config = Config::parse();
    
    println!("Config Loaded!");
    println!("User: {}, Port: {}", config.name, config.port);
}
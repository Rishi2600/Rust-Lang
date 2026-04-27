#[derive(Debug)]
struct ServerConfig {
    port: u16,
    timeout: u32,
    workers: u8,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { port: 8080, timeout: 30, workers: 4 }
    }
}

fn main() {
    // Start with the default, but override just the port
    let dev_config = ServerConfig {
        port: 3000,
        ..Default::default()
    };

    println!("Starting server: {:?}", dev_config);
}
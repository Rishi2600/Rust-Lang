use my_macros::AutoCli;

#[derive(AutoCli)]
struct Config {
    name: String,
    port: Option<u16>, // Must be exactly Option<u16>
}

fn main() {
    let config = Config::parse();
    println!("User: {}", config.name);
    if let Some(p) = config.port {
        println!("Port: {}", p);
    } else {
        println!("Port: Default (80)");
    }
}
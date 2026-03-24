use my_macros::AutoCli;

#[derive(AutoCli)]
struct Config {
    #[arg(short = "n")]
    name: String,
    
    #[arg(short = "p")]
    port: Option<u16>,

    #[arg(short = "v")]
    verbose: bool,
}

fn main() {
    let config = Config::parse();
    
    if config.verbose {
        println!("--- DEBUG MODE ENABLED ---");
    }
    
    println!("User: {}", config.name);
    println!("Port: {}", config.port.unwrap_or(80));
}
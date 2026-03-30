fn main() {
    let temperature = 35;

    match temperature {
        t if t > 40 => println!("Dangerously hot!"), // Guard
        30..=40 => println!("Beach weather."),       // Range
        15..=29 => println!("Perfect."),
        _ => println!("Too cold."),                  // The "Catch-all" (_)
    }
}
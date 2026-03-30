enum Status {
    Success(String),
    Error(u32),
    Retry { count: u8, reason: String },
}

fn main() {
    let logs = vec![
        Status::Success("Logged in".into()),
        Status::Retry { count: 3, reason: "Timeout".into() },
        Status::Error(404),
    ];

    for entry in logs {
        match entry {
            // Match a Success but only if it's not empty
            Status::Success(s) if !s.is_empty() => println!("Success: {}", s),
            
            // Match a Retry but only if we haven't tried too many times
            Status::Retry { count, .. } if count < 5 => println!("Retrying... (Attempt {})", count),
            
            // Match an Error and name the code 'e'
            Status::Error(e) => println!("Fatal Error Code: {}", e),
            
            // The "Catch-all" for anything else
            _ => println!("Ignoring minor event."),
        }
    }
}
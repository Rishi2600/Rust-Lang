use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // The '?' means: If File::open fails, return the error immediately.
    // If it succeeds, 'unwrap' the file and continue.
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s) // Success! Wrapped in the 'Ok' variant
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
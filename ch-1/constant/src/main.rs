use std::fmt::Display;

// This generic function creates zero abstraction runtime overhead
fn log_value<T: Display>(item: T) {
    println!("[LOG]: {}", item);
}

fn main() {
    // At compile-time, Rust sees two distinct type uses:
    log_value(100);          // 1. Generates: log_value_i32(item: i32)
    log_value("System OK");  // 2. Generates: log_value_str(item: &str)
    
    // Magic: At runtime, your binary directly executes the specialized functions, 
    // maximizing CPU execution cache locality.
}
use my_macros::time_it;

// 1. The function we want to measure
#[time_it]
fn heavy_computation() {
    println!("Starting heavy work...");
    std::thread::sleep(std::time::Duration::from_millis(50));
    println!("Work finished!");
}

// 2. The MANDATORY entry point
fn main() {
    println!("--- Program Start ---");
    
    // Call the function that has our macro attached
    heavy_computation();
    
    println!("--- Program End ---");
}
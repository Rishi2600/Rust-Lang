use my_macros::retry;

#[retry(3)]
fn fetch_data() -> Result<String, &'static str> {
    // In a real app, this might be a network request
    // Here, we simulate a constant failure to see the retry macro work
    println!("Attempting to fetch data...");
    Err("Network Timeout")
}

fn main() {
    println!("--- Starting Request ---");

    // Call the function wrapped by the macro
    match fetch_data() {
        Ok(data) => println!("Success: {}", data),
        Err(e) => println!("Final Error after retries: {}", e),
    }

    println!("--- Request Finished ---");
}
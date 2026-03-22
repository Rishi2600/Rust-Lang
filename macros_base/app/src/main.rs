use my_macros::log_with_target;

#[log_with_target("DATABASE")]
fn save_user() {
    println!("Saving to DB...");
}

#[log_with_target("NETWORK")]
fn fetch_api() {
    println!("Fetching from API...");
}

fn main() {
    save_user();
    fetch_api();
}
use my_macros::Describe; // The derive macro
use my_macros::log_call; // The attribute macro

#[derive(Describe)]
struct User;

#[log_call]
fn hello() {
    println!("Hello from the app!");
}

fn main() {
    hello();
}
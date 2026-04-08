use std::sync::Arc;
use std::thread;

fn main() {
    // We wrap a large string in an Arc
    let shared_data = Arc::new(String::from("Massive Database Record"));

    for i in 0..5 {
        let data_ref = Arc::clone(&shared_data);
        thread::spawn(move || {
            println!("Thread {} is reading: {}", i, data_ref);
        });
    }
}
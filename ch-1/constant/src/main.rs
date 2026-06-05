use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);

    // This thread will intentionally crash while holding the lock
    let handle = thread::spawn(move || {
        let mut data = counter_clone.lock().unwrap();
        *data = 42;
        panic!("Thread crashed abruptly!"); 
    });

    let _ = handle.join(); // Let the thread finish crashing

    // Trying to access the data from the main thread
    match counter.lock() {
        Ok(data) => println!("Counter data: {}", *data),
        Err(poisoned) => {
            println!("⚠️ Warning: The mutex was poisoned by a panicked thread!");
            // You can choose to recover the data anyway if you need to
            let recovered_data = poisoned.into_inner();
            println!("Recovered data safely: {}", *recovered_data);
        }
    }
}
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn increment_atomic(counter: &AtomicUsize) {
    let mut current = counter.load(Ordering::Relaxed);
    loop {
        let new_val = current + 1;
        // Compare-And-Swap: Atomically sets to 'new_val' ONLY if current memory matches 'current'
        match counter.compare_exchange_weak(current, new_val, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => break, // Successfully updated!
            Err(actual) => current = actual, // Another thread modified it first; update and retry loop
        }
    }
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            increment_atomic(&counter_clone);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Atomic Count: {}", counter.load(Ordering::SeqCst));
}
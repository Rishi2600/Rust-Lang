use std::sync::mpsc;
use std::thread;

fn main() {
    // tx = transmitter, rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Secret Message");
        tx.send(val).unwrap(); 
        // val is MOVED here; the thread can no longer use it!
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
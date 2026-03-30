enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Shutting down..."),
        Message::Write(text) => println!("Text: {}", text),
        Message::Move { x, y } => println!("Moving to x: {}, y: {}", x, y),
    }
}
enum Robot {
    Worker { id: u32, task: String },
}

fn main() {
    let bot = Robot::Worker { id: 777, task: String::from("Welding") };

    match bot {
        // 'entire_bot' binds to the whole Worker, while 'id' binds to the 777
        entire_bot @ Robot::Worker { id: 700..=800, .. } => {
            println!("Bot in the 700-series detected!");
            // We can still use 'entire_bot' here as the full object
            inspect_robot(entire_bot); 
        }
        _ => (),
    }
}

fn inspect_robot(r: Robot) { /* ... */ }
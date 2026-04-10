mod robot;
mod commands;

use robot::Robot;
use commands::{Action, MoveForward, Speak};

// The Macro: It recursively parses the DSL into Boxed Trait Objects
macro_rules! robot_program {
    // Match 'move_forward'
    (move_forward $dist:expr; $($rest:tt)*) => {
        queue.push(Box::new(MoveForward($dist)));
        robot_program!($($rest)*);
    };
    // Match 'speak'
    (speak $msg:expr; $($rest:tt)*) => {
        queue.push(Box::new(Speak($msg.to_string())));
        robot_program!($($rest)*);
    };
    // Base case: No more commands
    () => {};
}

fn main() {
    let mut r2d2 = Robot::new("R2-D2");
    let mut queue: Vec<Box<dyn Action>> = Vec::new();

    // Using our custom DSL!
    robot_program! {
        move_forward 10;
        speak "Beep Boop!";
        move_forward 5;
    }

    // Execute the compiled queue
    for cmd in queue {
        cmd.execute(&mut r2d2);
    }

    r2d2.status();
}
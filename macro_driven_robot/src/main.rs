mod robot;
mod commands;

use robot::Robot;
use commands::{Action, MoveForward, Speak};


macro_rules! robot_program {
    // Case: move_forward
    ($target:ident; move_forward $dist:expr; $($rest:tt)*) => {
        $target.push(Box::new(MoveForward($dist)));
        robot_program!($target; $($rest)*);
    };
    
    // Case: speak
    ($target:ident; speak $msg:expr; $($rest:tt)*) => {
        $target.push(Box::new(Speak($msg.to_string())));
        robot_program!($target; $($rest)*);
    };

    // Base Case: Ends the recursion when no tokens are left
    ($target:ident;) => {};
}

fn main() {
    // 1. Initialize our robot 
    let mut r2d2 = Robot::new("R2-D2");

    // 2. Create a collection of Trait Objects (Box<dyn Action>)
    // This allows us to store different types (Move and Speak) in one list.
    let mut queue: Vec<Box<dyn Action>> = Vec::new();

    // 3. Use the Macro DSL to populate the queue
    // Note: We MUST end each command with a semicolon as per our macro rules
    robot_program! {
        queue; 
        move_forward 10;
        speak "Beep Boop!";
        move_forward 5;
        speak "Objective complete.";
    }

    println!("--- Starting Robot Execution ---");

    // 4. Iterate and execute the commands
    // This is 'Dynamic Dispatch' in action!
    for cmd in queue {
        cmd.execute(&mut r2d2);
    }

    println!("--- Execution Finished ---");

    // 5. Check final state
    r2d2.status();
}
use crate::robot::Robot;

pub trait Action {
    fn execute(&self, robot: &mut Robot);
}

pub struct MoveForward(pub i32);
impl Action for MoveForward {
    fn execute(&self, robot: &mut Robot) {
        robot.y += self.0;
        robot.battery -= 5;
        println!("Moving forward {} units...", self.0);
    }
}

pub struct Speak(pub String);
impl Action for Speak {
    fn execute(&self, robot: &mut Robot) {
        println!("{} says: '{}'", robot.name, self.0);
    }
}
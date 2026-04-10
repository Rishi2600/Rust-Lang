pub struct Robot {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub battery: u8,
}

impl Robot {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            x: 0,
            y: 0,
            battery: 100,
        }
    }

    pub fn status(&self) {
        println!("[{}] Pos: ({}, {}), Battery: {}%", self.name, self.x, self.y, self.battery);
    }
}
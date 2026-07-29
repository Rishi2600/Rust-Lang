// A token representing exclusive access to an underlying system resource
pub struct HardwareBus;

pub struct ThermalSensor;

impl ThermalSensor {
    // To read the sensor, you MUST hand over full ownership of the HardwareBus token.
    // While this function runs, NO OTHER code can use the bus!
    pub fn read_temperature(&self, bus: HardwareBus) -> (f32, HardwareBus) {
        println!("Reading temp from hardware bus...");
        let temp = 36.6;
        
        // Return the bus token back to the caller so it can be reused
        (temp, bus)
    }
}

fn main() {
    let bus = HardwareBus;
    let sensor = ThermalSensor;

    // We pass 'bus' by value, transferring ownership into the function
    let (temp, bus) = sensor.read_temperature(bus);
    println!("Temp: {}°C", temp);

    // If we try to use 'bus' while it's inside another function:
    // let (temp2, bus) = sensor.read_temperature(bus); // Compiles fine because 'bus' was returned!
}
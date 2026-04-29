enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // A method that returns the duration for each state
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    // A method to get the next light in the sequence
    fn next(&self) -> Self {
        match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        }
    }
}
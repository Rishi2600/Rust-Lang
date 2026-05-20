// Defining unique types wrapped around f64
struct Meters(f64);
struct Seconds(f64);
struct Kmph(f64);

fn calculate_speed(distance: Meters, time: Seconds) -> Kmph {
    // We unwrap the inner value using tuple indexing (.0)
    let speed_mps = distance.0 / time.0;
    Kmph(speed_mps * 3.6)
}

fn main() {
    let dist = Meters(100.0);
    let time = Seconds(10.0);

    // This compiles perfectly
    let speed = calculate_speed(dist, time);
    println!("Speed: {} km/h", speed.0);

    // let bad_attempt = calculate_speed(time, dist); 
    // ❌ COMPILE ERROR: Expected Meters, found Seconds!
}
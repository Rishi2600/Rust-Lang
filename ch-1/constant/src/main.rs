use std::borrow::Cow;

fn sanitize_sensor_data<'a>(data: &'a [i32]) -> Cow<'a, [i32]> {
    // Check if there are any negative "error readings" in the slice
    if data.iter().any(|&x| x < 0) {
        // We found bad data! Allocate a new Vec, modify it, and return an Owned version.
        let cleaned: Vec<i32> = data.iter().map(|&x| if x < 0 { 0 } else { x }).collect();
        Cow::Owned(cleaned)
    } else {
        // Data is clean. Magic: Return a direct reference wrapped in Cow. Zero allocations!
        Cow::Borrowed(data)
    }
}

fn main() {
    let clean_readings = [22, 25, 24, 26];
    let dirty_readings = [21, -999, 23, 25]; // -999 is a sensor error

    // Flow 1: Zero allocation, lightning fast reference passing
    let result1 = sanitize_sensor_data(&clean_readings);
    println!("Clean result (Is Borrowed: {}): {:?}", matches!(result1, Cow::Borrowed(_)), result1);

    // Flow 2: Allocates a new vector only because it's required
    let result2 = sanitize_sensor_data(&dirty_readings);
    println!("Dirty result (Is Owned: {}): {:?}", matches!(result2, Cow::Owned(_)), result2);
}
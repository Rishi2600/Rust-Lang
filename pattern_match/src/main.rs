fn main() {
    let system_power = true;
    let authorization_level = 10;

    // We match on the PAIR of values
    match (system_power, authorization_level) {
        (true, 10) => println!("Full Access: Critical Systems Online."),
        (true, _)  => println!("Limited Access: Systems Online."),
        (false, _) => println!("Systems Offline. Emergency power only."),
    }
}
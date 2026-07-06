use std::any::Any;

fn process_unknown_type(value: &dyn Any) {
    // Attempt to downcast the dynamic reference back into a concrete String
    if let Some(string_ref) = value.downcast_ref::<String>() {
        println!("Magic: Discovered a String value: '{}'", string_ref);
    } 
    // Attempt to downcast into an i32 instead
    else if let Some(int_ref) = value.downcast_ref::<i32>() {
        println!("Magic: Discovered an i32 value: {}", int_ref);
    } else {
        println!("Unknown type passed.");
    }
}

fn main() {
    let my_string = String::from("Hello Dynamic World");
    let my_number = 42;

    process_unknown_type(&my_string);
    process_unknown_type(&my_number);
}
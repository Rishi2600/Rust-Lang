fn main() {
    let final_result = add_numbers(3, 4);
    println!("{}", final_result);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    // --- This is a STATEMENT ---
    // It performs an action (printing) but does not return a value.
    println!("Adding numbers!");

    // --- This is a STATEMENT ---
    // It performs an action (creating a variable).
    let result = x + y;

    // --- This is an EXPRESSION ---
    // It has no semicolon. It evaluates to the value of `result`.
    // This value is returned from the function.
    result
}
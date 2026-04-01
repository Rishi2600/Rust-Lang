fn main() {
    let sequence = [1, 2, 3, 4, 5];

    match sequence {
        // Capture the first, capture the last, and ignore the middle
        [first, .., last] => {
            println!("The bridge starts at {} and ends at {}", first, last);
        }
    }
    
    let path = ["home", "user", "documents", "secret.txt"];
    match path {
        // Match the directory structure and the filename separately
        [dirs @ .., filename] => {
            println!("Filename: {}", filename);
            println!("Full directory path: {:?}", dirs);
        }
    }
}
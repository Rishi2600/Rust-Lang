// This function accepts a slice. It works on Vectors, Fixed Arrays, or Sub-slices!
fn print_first_two(items: &[i32]) {
    if let [first, second, ..] = items {
        println!("First two items are: {} and {}", first, second);
    } else {
        println!("Not enough items!");
    }
}

fn main() {
    let my_vector: Vec<i32> = vec![10, 20, 30];
    let my_array: [i32; 4] = [100, 200, 300, 400];

    // 1. Pass a vector as a slice
    print_first_two(&my_vector);

    // 2. Pass a fixed array as a slice
    print_first_two(&my_array);

    // 3. Pass a SUB-SLICE (just elements 200 and 300 from the array)
    print_first_two(&my_array[1..3]);
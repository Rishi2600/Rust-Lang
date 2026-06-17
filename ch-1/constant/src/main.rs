// This function runs AT COMPILE TIME if passed a constant expression
const fn compute_lookup_table(size: usize) -> [usize; 5] {
    let mut table = [0; 5];
    let mut i = 0;
    while i < 5 {
        table[i] = i * size;
        i += 1;
    }
    table
}

fn main() {
    // Magic: This function is executed by your CPU while you run 'cargo build'!
    // The final array [0, 10, 20, 30, 40] is embedded straight into the machine code.
    const MY_TABLE: [usize; 5] = compute_lookup_table(10);

    println!("Lookup table: {:?}", MY_TABLE);
}
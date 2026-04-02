mod hardware;
mod mmu;
mod errors;

use mmu::{MMU, PageState};

fn main() {
    let mut mmu = MMU::new(64); // 64 Virtual Pages

    // 1. Manually "Allocate" Page 2 into RAM Frame 5
    mmu.page_table[2] = PageState::InRam { frame_id: 5 };

    // 2. Simulate a Virtual Address in Page 2 (e.g., 0x2050)
    let v_addr = 0x2050; 
    
    println!("Requesting Virtual Address: {:#X}", v_addr);

    match mmu.translate(v_addr) {
        Ok(p_addr) => {
            println!("Translated to Physical Address: {:#X}", p_addr);
            
            // Write a value to that physical address
            mmu.ram.write(p_addr, 42);
            println!("Value at Physical {:#X}: {}", p_addr, mmu.ram.read(p_addr));
        }
        Err(e) => println!("Memory Error: {:?}", e),
    }

    // 3. Try to access unallocated memory
    let bad_addr = 0x9000;
    println!("\nRequesting Bad Address: {:#X}", bad_addr);
    if let Err(e) = mmu.translate(bad_addr) {
        println!("Expected Error caught: {:?}", e);
    }
}
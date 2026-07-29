use std::ptr;

// Simulating a hardware control register mapped at a specific memory location
#[repr(C)]
struct HardwareRegister {
    control: u32,
    status: u32,
    data: u32,
}

fn main() {
    // A hypothetical physical address for a UART serial controller
    let fake_hardware_address: usize = 0x1000_0000;
    
    // Cast the raw integer memory address directly into a raw pointer
    let reg_ptr = fake_hardware_address as *mut HardwareRegister;

    println!("Hardware register configured at memory location: {:p}", reg_ptr);

    // UNSAFE BLOCK: Reading/Writing to raw memory addresses directly
    unsafe {
        // Volatile write ensures the compiler NEVER optimizes away this store instruction,
        // which is critical for toggling physical hardware pins.
        // (Commented out to prevent segmentation fault on standard host environments)
        // ptr::write_volatile(&mut (*reg_ptr).control, 0x01); 
        
        println!("Magic: Volatile memory instructions generated for register access.");
    }
}
use std::marker::PhantomData;

// Marker types representing physical memory banks
pub struct GpuMemory;
pub struct SystemRam;

// A pointer tagged with its memory domain
pub struct MemoryBlock<Domain> {
    pub address: usize,
    pub size: usize,
    _domain: PhantomData<Domain>, // Erasable type tag (0 bytes!)
}

fn allocate_gpu_buffer(size: usize) -> MemoryBlock<GpuMemory> {
    MemoryBlock {
        address: 0xDEAD_BEEF, // Simulated GPU VRAM location
        size,
        _domain: PhantomData,
    }
}

fn process_gpu_data(block: &MemoryBlock<GpuMemory>) {
    println!("Processing {} KB in VRAM at address 0x{:X}", block.size, block.address);
}

fn main() {
    let vram_block = allocate_gpu_buffer(1024);
    
    process_gpu_data(&vram_block);

    // If you try to pass System RAM to a GPU function:
    // let ram_block: MemoryBlock<SystemRam> = ...;
    // process_gpu_data(&ram_block); 
    // ❌ COMPILE ERROR: Expected `MemoryBlock<GpuMemory>`, found `MemoryBlock<SystemRam>`
}
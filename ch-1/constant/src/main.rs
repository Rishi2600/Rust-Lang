// Requires the `tikv-jemallocator` crate in Cargo.toml
// use tikv_jemallocator::Jemalloc;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    // All allocations here automatically use the custom high-throughput allocator
    let mut data = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        data.push(i);
    }
    println!("Allocated {} items using custom global allocator.", data.len());
}
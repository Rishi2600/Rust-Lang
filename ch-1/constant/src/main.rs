// Guarantees that at the assembly level, UserId is exactly identical to a raw u64
#[repr(transparent)]
struct UserId(u64);

fn process_raw_ids(ids: &[u64]) {
    println!("Processing {} ids at address: {:p}", ids.len(), ids.as_ptr());
}

fn main() {
    let my_user_ids: Vec<UserId> = vec![UserId(1), UserId(2), UserId(3)];

    // Magic: Because of repr(transparent), we can unsafely cast a slice of 
    // UserId directly into a slice of u64 instantly with ZERO copies.
    let raw_ids: &[u64] = unsafe {
        std::slice::from_raw_parts(my_user_ids.as_ptr() as *const u64, my_user_ids.len())
    };

    process_raw_ids(raw_ids);
}
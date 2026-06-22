use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfReferential {
    value: String,
    // This pointer will target our own `value` field
    internal_ptr: *const String,
    // Opt-out of the 'Move' trait marker, forcing this struct to be unmovable once pinned
    _marker: PhantomPinned, 
}

fn main() {
    // 1. Initialize a self-referential layout safely on the heap
    let mut unpinned = Box::pin(SelfReferential {
        value: String::from("SafeData"),
        internal_ptr: std::ptr::null(),
        _marker: PhantomPinned,
    });

    // 2. Unsafely configure the raw pointer to target our own internal string address
    unsafe {
        let heap_ref = unpinned.as_mut().get_unchecked_mut();
        heap_ref.internal_ptr = &heap_ref.value;
        
        // Magic: Pin prevents you from doing things like swapping or moving 
        // this object out of the box, preserving the raw pointer address safely.
        println!("Value via internal pointer: {}", *heap_ref.internal_ptr);
    }
}
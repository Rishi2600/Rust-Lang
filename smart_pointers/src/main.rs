fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(2);

    for i in 0..10 {
        println!(
            "Item {}: Address: {:p}, Len: {}, Cap: {}", 
            i, v.as_ptr(), v.len(), v.capacity()
        );
        v.push(i);
    }
}
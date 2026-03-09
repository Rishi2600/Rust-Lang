fn main() {
    // An infinite range of squares, but we only take the first 5
    let squares: Vec<_> = (1..).map(|x| x * x).take(5).collect();
    
    println!("{:?}", squares); // [1, 4, 9, 16, 25]
}
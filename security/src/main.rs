trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle { fn draw(&self) { println!("◯"); } }
impl Drawable for Square { fn draw(&self) { println!("□"); } }

fn main() {
    // We store different types in one Vec using "Dynamic Dispatch"
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes {
        shape.draw();
    }
}
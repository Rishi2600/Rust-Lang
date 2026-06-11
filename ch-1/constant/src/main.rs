// 1. Define distinct state markers
struct Empty;
struct Full;

// 2. The struct is generic over its state
struct BoxContainer<State> {
    content: Option<String>,
    _marker: std::marker::PhantomData<State>, // Tells the compiler this generic is structural
}

// 3. Methods ONLY available when the container is Empty
impl BoxContainer<Empty> {
    fn new() -> Self {
        Self { content: None, _marker: std::marker::PhantomData }
    }

    fn fill(self, item: &str) -> BoxContainer<Full> {
        BoxContainer {
            content: Some(item.to_string()),
            _marker: std::marker::PhantomData,
        }
    }
}

// 4. Methods ONLY available when the container is Full
impl BoxContainer<Full> {
    fn consume(self) -> String {
        self.content.unwrap()
    }
}

fn main() {
    let container = BoxContainer::new();
    
    // let item = container.consume(); 
    // ❌ COMPILE ERROR: No method named `consume` found for BoxContainer<Empty>

    let filled_container = container.fill("Super Weapon");
    let item = filled_container.consume(); // Compiles perfectly!
}
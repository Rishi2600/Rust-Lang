trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;
struct FileLogger { path: String }

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) { println!("[Console]: {}", message); }
}

// Superpower: Using a Generic with a Trait Bound
fn run_app<L: Logger>(logger: L) {
    logger.log("Application started!");
}

fn main() {
    let logger = ConsoleLogger;
    run_app(logger); // Works with any type that implements Logger
}
// This struct and implementation only exist if compiling for a Linux system
#[cfg(target_os = "linux")]
fn print_platform_message() {
    println!("Running on a Linux native kernel environment.");
}

// This struct and implementation only exist if compiling for a Windows system
#[cfg(target_os = "windows")]
fn print_platform_message() {
    println!("Running on a Windows operating system environment.");
}

fn main() {
    // Magic: The compiler resolves which function block exists 
    // BEFORE translating any syntax into machine instructions.
    print_platform_message();
}
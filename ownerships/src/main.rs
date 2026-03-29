use std::sync::{Arc, Mutex};
use std::thread;

struct Spell<'a> {
    name: &'a str, // Level 4: Lifetime 'a (The spell name lives as long as the string literal)
}

fn main() {
    // Level 8: Arc (Atomic Reference Counting) - Multiple owners across threads
    // Level 7: Mutex (Mutual Exclusion) - Interior mutability that is thread-safe
    let reactor_core = Arc::new(Mutex::new(100)); 
    
    let incantation = Spell { name: "Fireball" };
    let mut wizards = vec![];

    for i in 0..3 {
        // Clone the Arc, not the data. We now have 4 "owners" of the same Mutex.
        let core_ref = Arc::clone(&reactor_core);
        
        // Level 6: The 'move' Closure. We move 'core_ref' into the thread.
        let handle = thread::spawn(move || {
            // Level 3 & 7: Lock the Mutex to get a mutable reference (&mut)
            let mut mana = core_ref.lock().unwrap();
            
            if *mana >= 20 {
                *mana -= 20;
                println!("Wizard {} cast {}! Core Mana: {}", i, incantation.name, *mana);
            } else {
                println!("Wizard {} failed! Not enough mana.", i);
            }
            // Mana is automatically unlocked here when the Guard goes out of scope.
        });
        
        wizards.push(handle);
    }

    // Wait for all wizards to finish
    for wizard in wizards {
        wizard.join().unwrap();
    }

    // Final check of the owner count
    println!("Reactor stabilized. Final Mana: {}", reactor_core.lock().unwrap());
}
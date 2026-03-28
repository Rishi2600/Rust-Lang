struct Wizard<'a> {
    name: &'a str, // This wizard is "anchored" to a string slice
}

fn main() {
    let name = String::from("Gandalf");
    let wiz;

    {
        let _temporary_name = String::from("Saruman");
        wiz = Wizard { name: &name }; 
        // Because 'temporary_name' dies at the end of this brace.
    }
    
    println!("{}", wiz.name); 
}
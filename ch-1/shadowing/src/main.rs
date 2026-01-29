fn main() {
    let x = 5;

    x = x + 100;

    {
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in thesecond  inner scope is: {x}");
        }

        println!("The value of x in first inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
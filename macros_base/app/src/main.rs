use my_macros::html;

fn main() {
    let website = html!(
        div {
            h1 { "Welcome to my DSL" }
            p {
                "This is "
                span { "nested" }
                " content!"
            }
        }
    );

    println!("{}", website);
}
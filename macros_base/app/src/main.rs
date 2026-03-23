use my_macros::html;

fn main() {
    let page = html!(
        div(class="container", id="main-wrapper") {
            h1(style="color: blue") { "Hello Attributes!" }
            p {
                "Check out this "
                a(href="https://rust-lang.org") { "Link" }
            }
        }
    );

    println!("{}", page);
}
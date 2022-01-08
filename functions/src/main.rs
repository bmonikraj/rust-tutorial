fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    for x in (1..7).rev() {
        println!("Another function! {}", x);
    }
}
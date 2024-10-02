//declarative_macro.rs

macro_rules! greetings {
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

fn main() {
    greetings!("Earthly"); // Prints "Hello, Earthly"
}
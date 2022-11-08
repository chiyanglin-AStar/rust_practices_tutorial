fn main() {
    let mut s;
    s = greet();
    println!("{s}");
}

fn greet() -> String {
    "Hello, world!".to_string()
}

#[test] // test attribute indicates, this is a test function
fn test_greet() {
    assert_eq!("Hello, world!", greet())
}

// 💡 Always put test functions inside a tests module with #[cfg(test)] attribute. 
// cfg(test) module compiles only when running tests. We discuss more about this in next section.


fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

fn main() {
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    let ss = "Hello".to_string();
    takes_slice(&ss);
}

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");

    println!("You entered: {}", num);
}
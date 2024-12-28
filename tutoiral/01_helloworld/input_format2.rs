use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer, a float, and a string (separated by spaces):");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut inputs = input.trim().split_whitespace();
    let int_value: i32 = inputs.next().unwrap().parse().expect("Invalid integer");
    let float_value: f64 = inputs.next().unwrap().parse().expect("Invalid float");
    let string_value = inputs.next().unwrap_or("Default");

    println!("\nYou entered:");
    println!("Integer: {}", int_value);
    println!("Float: {}", float_value);
    println!("String: {}", string_value);
}
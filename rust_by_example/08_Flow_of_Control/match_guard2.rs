fn main() {
    let number = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("other"),

        // _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}
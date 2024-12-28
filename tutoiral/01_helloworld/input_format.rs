use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if sufficient arguments are passed
    if args.len() != 4 {
        println!("Usage: {} <integer> <float> <string>", args[0]);
        return;
    }

    // Parse Integer Argument
    let int_arg: i32 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: The first argument must be an integer.");
            return;
        }
    };

    // Parse Float Argument
    let float_arg: f64 = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: The second argument must be a float.");
            return;
        }
    };

    // Parse String Argument (No parsing needed for strings)
    let string_arg: String = args[3].clone();

    // Print the parsed values
    println!("Integer Argument: {}", int_arg);
    println!("Float Argument: {}", float_arg);
    println!("String Argument: {}", string_arg);
}
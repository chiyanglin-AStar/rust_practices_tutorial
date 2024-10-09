
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

#### [ch06_Conversion](https://doc.rust-lang.org/rust-by-example/conversion.html) 
    
- [console input integer](https://medium.com/@rohanbhatotiya/how-can-we-take-integers-as-an-input-in-rust-8f76ddf51010)

-[How to read an integer from stdin](https://users.rust-lang.org/t/how-to-read-an-integer-from-stdin/57538/17)

Example:

    fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is 
    :{}",cmd_line.len()); 
    // total number of elements passed

    let mut sum = 0;
    let mut has_read_first_arg = false;

    //iterate through all the arguments and calculate their sum

    for arg in cmd_line {
        if has_read_first_arg { //skip the first argument since it is the exe file name
            sum += arg.parse::<i32>().unwrap();
        }
        has_read_first_arg = true; 
        // set the flag to true to calculate sum for the subsequent arguments.
    }
    println!("sum is {}",sum);
    }

Example:

    fn main(){
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        //input.trim().parse().expect("Invalid input")
        //let parsed: i32 = line.parse().unwrap();
        //let mut a =0;
        //let a1 = std::io::stdin().read_line(&mut a).unwrap();
        println!("Hello, World.");
        println!("{}", line);
        println!("b1 ==> {}", line);
    
        //println!("a1+100 = {}", a1+100);
    }

Example:

    use std::io;
    fn main() {
    let lines = io::stdin().lines();
        for line in lines {
            println!("got a line: {}", line.unwrap());
        }
    }

/*

io::stdin() // the rough equivalent of `std::cin`
    .read_line(&mut input_line) // actually read the line
    .expect("Failed to read line"); // which can fail, however
let x: i32 = input_line
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer"); // which, again, can fail
    
*/

// Enter your code here 
fn main(){
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let parsed: i32 = line.parse().unwrap();
    //let mut a =0;
    //let a1 = std::io::stdin().read_line(&mut a).unwrap();
    println!("Hello, World.");
    println!("{}", line);
    println!("b1 ==> {}", line);
    println!("{}", parsed);
     //println!("a1+100 = {}", a1+100);
 }

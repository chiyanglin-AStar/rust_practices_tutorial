// Enter your code here 
fn main(){
    let mut line_str  = String::new();
    let mut line_str2 = String::new();
    let b1 = std::io::stdin().read_line(&mut line_str).unwrap();
     let num1: i32 = line_str.trim().parse().expect("Invalid input");
     
     let b2 = std::io::stdin().read_line(&mut line_str2).unwrap();
     let num2:i32 = line_str2.trim().parse().expect("Invalid input");
 
     let sum = num1 + num2;
     
     println!("Hello, World.");
     println!("line_str = {}", line_str);
     println!("b1 ==> {}", line_str);
     
     println!("Sum: {:?}", sum);
 }

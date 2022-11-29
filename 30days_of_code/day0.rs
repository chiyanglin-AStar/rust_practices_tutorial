// Enter your code here
fn main(){
   let mut line = String::new();
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hello, World.");
   println!("{}", line);
}

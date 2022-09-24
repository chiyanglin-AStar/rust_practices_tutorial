fn main() {
   let n1 = "Tutorials".to_string();
   println!("length of string is {}",n1.len());
   let c1 = &n1[4..9]; 
   
   // fetches characters at 4,5,6,7, and 8 indexes
   println!("{}",c1);
}

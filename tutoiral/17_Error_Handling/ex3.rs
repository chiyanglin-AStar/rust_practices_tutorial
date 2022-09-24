use std::fs::File;
fn main() {
   let f = File::open("main.jpg"); 
   //this file does not exist
   println!("{:?}",f);
}

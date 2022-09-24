use std::fs::File;
fn main(){
   let f = File::open("pqr.txt").expect("File not able to open");
   //file does not exist
   println!("end of main");
}

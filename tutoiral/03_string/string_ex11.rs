fn main() {
   let fullname = " Tutorials Point \r\n";
   println!("Before trim ");
   println!("fullname = {} ",fullname);
   println!("length is {}",fullname.len());
   println!();
   println!("After trim ");
   println!("fullname = {} ",fullname.trim());
   println!("length is {}",fullname.trim().len());
}

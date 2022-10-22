fn main() {
   const NAME:&str = "Mohtashim";
   const NAME1:usize = NAME.len(); 
   //Error : `NAME` already defined
   println!(" name length = {}",NAME.len());
   println!(" name length = {}",NAME1);
   println!("name changed to integer : {}",NAME);
}

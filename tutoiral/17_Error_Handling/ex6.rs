fn main(){
   let result = is_even(13);
   match result {
      Ok(d)=>{
         println!("no is even {}",d);
      },
      Err(msg)=>{
         println!("Error msg is {}",msg);
      }
   }
   println!("end of main");
}
fn is_even(no:i32)->Result<bool,String> {
   if no%2==0 {
      return Ok(true);
   } else {
      return Err("NOT_AN_EVEN".to_string());
   }
}

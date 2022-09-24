use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("size of map is {}",stateCodes.len());
   println!("{:?}",stateCodes);

   match stateCodes.get(&"KL") {
      Some(value)=> {
         println!("Value for key KL is {}",value);
      }
      None => {
         println!("nothing found");
      }
   }
}

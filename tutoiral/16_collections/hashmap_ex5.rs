use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");

   for (key, val) in stateCodes.iter() {
      println!("key: {} val: {}", key, val);
   }
}

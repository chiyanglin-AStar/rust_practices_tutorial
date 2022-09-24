use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   stateCodes.insert("GJ","Gujarat");

   println!("length of the hashmap {}",stateCodes.len());
   stateCodes.remove(&"GJ");
   println!("length of the hashmap after remove() {}",stateCodes.len());
}

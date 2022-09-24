use std::collections::HashMap;
fn main() {
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   stateCodes.insert("GJ","Gujarat");

   if stateCodes.contains_key(&"GJ") {
      println!("found key");
   }
}

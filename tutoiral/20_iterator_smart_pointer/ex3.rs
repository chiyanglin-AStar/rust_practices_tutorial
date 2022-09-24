fn main() {
   let names = vec!["Kannan", "Mohtashim", "Kiran"];
   for name in names.iter() {
      match name {
         &"Mohtashim" => println!("There is a rustacean among us!"),
         _ => println!("Hello {}", name),
      }
   }
   println!("{:?}",names); 
   // reusing the collection after iteration
}

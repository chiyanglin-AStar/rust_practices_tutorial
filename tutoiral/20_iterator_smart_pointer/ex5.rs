fn main(){
   let names = vec!["Kannan", "Mohtashim", "Kiran"];
   for name in names.into_iter() {
      match name {
         "Mohtashim" => println!("There is a rustacean among us!"),
         _ => println!("Hello {}", name),
      }
   }
   // cannot reuse the collection after iteration
   //println!("{:?}",names); 
   //Error:Cannot access after ownership move
}

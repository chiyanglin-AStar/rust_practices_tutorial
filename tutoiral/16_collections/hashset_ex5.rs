use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");

   match names.get(&"Mohtashim"){
      Some(value)=>{
         println!("found {}",value);
      }
      None =>{
         println!("not found");
      }
   }
   println!("{:?}",names);
}



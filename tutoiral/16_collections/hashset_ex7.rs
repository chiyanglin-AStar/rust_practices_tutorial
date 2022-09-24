use std::collections::HashSet;

fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");

   if names.contains(&"Kannan") {
      println!("found name");
   }  
}

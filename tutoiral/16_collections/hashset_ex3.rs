use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");

   for name in names.iter() {
      println!("{}",name);
   }
}

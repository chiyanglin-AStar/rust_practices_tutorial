use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();

   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   names.insert("Mohtashim");//duplicates not added

   println!("{:?}",names);
}

use std::collections::HashSet;
fn main() {
   let mut names = HashSet::new();
   names.insert("Mohtashim");
   names.insert("Kannan");
   names.insert("TutorialsPoint");
   println!("size of the set is {}",names.len());
}

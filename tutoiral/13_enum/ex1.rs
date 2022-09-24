// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
   Male,Female
}
fn main() {
   let male = GenderCategory::Male;
   let female = GenderCategory::Female;

   println!("{:?}",male);
   println!("{:?}",female);
}

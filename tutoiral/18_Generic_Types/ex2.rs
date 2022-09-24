fn main() {
   let mut vector_integer: Vec<i32> = vec![20,30];
   vector_integer.push(40);
   vector_integer.push("hello"); 
   //error[E0308]: mismatched types
   println!("{:?}",vector_integer);
}

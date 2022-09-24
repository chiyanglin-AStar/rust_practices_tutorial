fn main() {
   //declare an array
   let a = [10,20,30];

   let mut iter = a.iter(); 
   // fetch an iterator object for the array
   println!("{:?}",iter);

   //fetch individual values from the iterator object
   println!("{:?}",iter.next());
   println!("{:?}",iter.next());
   println!("{:?}",iter.next());
   println!("{:?}",iter.next());
}

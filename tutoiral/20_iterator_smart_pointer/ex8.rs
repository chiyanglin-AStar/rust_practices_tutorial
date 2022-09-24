fn main(){
   let val = 10; 
   // declared outside
   let closure2 = |x| {
      x + val //inner function accessing outer fn variable
   };
   println!("{}",closure2(2));
}

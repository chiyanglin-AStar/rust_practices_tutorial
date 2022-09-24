fn main() {
   let age:u8 = 255;

   // 0 to 255 only allowed for u8
   let weight:u8 = 256;   //overflow value is 0
   let height:u8 = 257;   //overflow value is 1
   let score:u8 = 258;    //overflow value is 2

   println!("age is {} ",age);
   println!("weight is {}",weight);
   println!("height is {}",height);
   println!("score is {}",score);
}

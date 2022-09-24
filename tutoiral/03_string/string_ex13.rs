fn main() {
   let fullname = "Kannan,Sudhakaran,Tutorialspoint";

   for token in fullname.split(","){
      println!("token is {}",token);
   }

   //store in a Vector
   println!("\n");
   let tokens:Vec<&str>= fullname.split(",").collect();
   println!("firstName is {}",tokens[0]);
   println!("lastname is {}",tokens[1]);
   println!("company is {}",tokens[2]);
}

pub mod movies {
   pub fn play(name:String) {
      println!("Playing movie {}",name);
   }
}
fn main(){
   movies::play("Herold and Kumar".to_string());
}

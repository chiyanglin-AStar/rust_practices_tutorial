// 01. Calling private functions of the same module
fn main() {
  phrases::greet();
}

mod phrases {
  pub fn greet() {
    hello(); //or self::hello();
  }
  
  fn hello() {
    println!("Hello, world!");
  }
}

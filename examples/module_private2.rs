// 02. Calling private functions of the parent module
fn main() {
  phrases::greetings::hello();
}

mod phrases {
  fn private_fn() {
    println!("Hello, world!");
  }
  
  pub mod greetings {
    pub fn hello() {
      super::private_fn();
    }
  }
}

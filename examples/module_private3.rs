fn main() {
  greetings::hello();
}

fn hello() {
  println!("Hello, world!");
}

mod greetings {
  pub fn hello() {
    super::hello();
  }
}

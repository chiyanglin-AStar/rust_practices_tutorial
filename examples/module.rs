fn main() {
   greetings::hello();
}

mod greetings {
  // ⭐️ By default, everything inside a module is private
  pub fn hello() { // ⭐️ So function has to be public to access from outside
    println!("Hello, world!");
  }
}

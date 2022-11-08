fn main() { 
  phrases::greetings::hello();
}

mod phrases { 
  pub mod greetings { 
    pub fn hello() { 
      println!("Hello, world!");
    }
  }
}

fn greeting<'a>() -> &'a str {
  "Hi!"
}


fn fullname<'a>(fname: &'a str, lname: &'a str) -> String {
  format!("{} {}", fname, lname)
}


struct Person<'a> { 
    fname: &'a str,
    lname: &'a str
}
  impl<'a> Person<'a> {
      fn new(fname: &'a str, lname: &'a str) -> Person<'a> { //no need to specify <'a> after new; impl already has it
          Person {
              fname : fname,
              lname : lname
          }
      }

      fn fullname(&self) -> String {
          format!("{} {}", self.fname , self.lname)
      }
  }

fn main() {
    let player = Person::new("Serena", "Williams");
    let player_fullname = player.fullname();
    
    println!("Player: {}", player_fullname);
}

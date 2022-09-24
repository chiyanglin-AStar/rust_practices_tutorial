//declare a structure
struct Employee {
   name:String,
   company:String,
   age:u32
}
fn main() {
   //initialize a structure
   let emp1 = Employee {
      company:String::from("TutorialsPoint"),
      name:String::from("Mohtashim"),
      age:50
   };
   let emp2 = Employee{
      company:String::from("TutorialsPoint"),
      name:String::from("Kannan"),
      age:32
   };
   //pass emp1 and emp2 to display()
   display(emp1);
   display(emp2);
}
// fetch values of specific structure fields using the 
// operator and print it to the console
fn display( emp:Employee){
   println!("Name is :{} company is {} age is 
   {}",emp.name,emp.company,emp.age);
}

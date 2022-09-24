fn main() {
   let mut name:String = String::from("TutorialsPoint");
   display(&mut name); 
   //pass a mutable reference of name
   println!("The value of name after modification is:{}",name);
}
fn display(param_name:&mut String){
   println!("param_name value is :{}",param_name);
   param_name.push_str(" Rocks"); 
   //Modify the actual string,name
}

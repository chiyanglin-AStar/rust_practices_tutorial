/*
    ./console_input 01 02 03
*/
fn main(){
    let cmd_line = std::env::args();
    println!("No of elements in arguments is :{}",cmd_line.len()); 
    //println!("content  in arguments is {:?}",cmd_line); 
}

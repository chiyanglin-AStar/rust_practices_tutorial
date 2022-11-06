
//Function with arguments 
fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}

//Returning
fn plus_one(a: i32) -> i32 {
    a + 1
    // There is no ending ; in the above line. It means this is an expression which equals to `return a+1;`
}
/*
fn plus_two(a: i32) -> i32 {
    return a + 2; //return a+2 but bad practice, 
    //should use only on conditional returnes, except it's last expression 
}
*/

fn plus_two(b: fn(i32) -> i32, x: i32) -> i32 {
    b(b(x))
}

//Hello world function
fn main() {
    // ⭐️ Function pointers, Usage as a Data Type
    let b = plus_one;
    let c = b(5); //6

    let b: fn(i32) -> i32 = plus_one; //same, with type inference
    let c = b(5); //6

    let b = plus_one;
    
    println!("{}", plus_two(b, 2)); //4
    
    println!("Hello, world!");
}

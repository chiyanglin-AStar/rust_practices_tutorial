fn main() {
    let a = "Hello, world."; //a: &'static str
    let b: &str = "こんにちは, 世界!";
    
    println!("a string");
    println!("{:?}", a); 
    println!("{:#?}",a);
    
    println!("b string");
    println!("{:?}", b); 
    println!("{:#?}",b);
}

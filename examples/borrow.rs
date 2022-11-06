fn main() {

    /*
    let a = [1, 2, 3];
    let b = a;
    
    let a = vec![1, 2, 3];
    let b = a;
    
    let mut a = vec![1, 2, 3];
    let b = &mut a; 
    
    println!("{:?} {:?}", a, b); // [1, 2, 3] [1, 2, 3]
    
    let a = [1, 2, 3];
    let b = &a;
    println!("{:?} {}", a, b[0]); // [1, 2, 3] 1
    
    */
    let a = [1, 2, 3];
    let b = &a;
    println!("{:?} {}", a, b[0]); // [1, 2, 3] 1
}

fn main() {
    //Creating vectors - 2 ways
    //let mut a = Vec::new(); //1.with new() keyword
    //let mut b = vec![]; //2.using the vec! macro

    //Creating with data types
    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![];
    let mut b3 = vec![1i32, 2, 3];//sufixing 1st value with data type
    println!("b3 = ");
    println!("{:?}", b3);
    //Creating with data
    let mut b4 = vec![1, 2, 3];
    let mut b5: Vec<i32> = vec![1, 2, 3];
    let mut b6  = vec![1i32, 2, 3];
    let mut b7 = vec![0; 10]; //ten zeroes
    println!("b7 = ");
    println!("{:?}", b7);
    //Accessing and changing exsisting data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    //c[6] = 2; can't assign values this way, index out of bounds
    println!("{:?}", c); //[1, 2, 3, 2, 1]

    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop(); //[1] : : Remove an element from the end
    println!("d = ");
    println!("{:?}", d);
    
    // 🔎 Capacity and reallocation
    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

    // These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }
    // ...but this may make the vector reallocate
    e.push(11);
    println!("e = ");
    println!("{:?}", e);
}

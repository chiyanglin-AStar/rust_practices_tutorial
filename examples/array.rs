fn main() {
    //  Array :

    let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    let mut b = [1, 2, 3];

    let c: [i32; 3] = [1, 2, 3]; //[Type; NO of elements]

    let d: [String; 3]; //["my value", "my value", "my value"];

    let e: [i32; 0] = []; //empty array
    println!("a array:");
    println!("{:?}", a); //[1, 2, 3]
    println!("{:#?}",a);
    
    println!("b array:");
    println!("{:?}", b); 
    println!("{:#?}",b);
    
    println!("c array:");
    println!("{:?}", c); 
    println!("{:#?}",c);
}

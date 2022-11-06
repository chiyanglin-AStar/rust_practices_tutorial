fn main() {
    //  Slice 

    let a: [i32; 4] = [1, 2, 3, 4]; //Parent Array

    let b: &[i32] = &a;             //Slicing whole array
    let c = &a[0..4];               // From 0th position to 4th(excluding)
    let d = &a[..];                 //Slicing whole array

    let e = &a[1..3];               //[2, 3]
    let e = &a[1..];                //[2, 3, 4]
    let e = &a[..3];                //[1, 2, 3]
    
    println!("b from a slice array:");
    println!("{:?}", b); //[1, 2, 3]
    println!("{:#?}",b);
}

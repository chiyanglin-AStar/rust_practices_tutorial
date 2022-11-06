fn main() {
    //  Array :

    let a = (1, 1.5, true, 'a', "Hello, world!");
    // a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello, world!"

    let b: (i32, f64) = (1, 1.5);

    let (c, d) = b; // c = 1, d = 1.5
    let (e, _, _, _, f) = a; //e = 1, f = "Hello, world!", _ indicates not interested of that item

    let g = (0,); //single-element tuple

    let h = (b, (2, 4), 5); //((1, 1.5), (2, 4), 5)
    println!("a tuple:");
    println!("{:?}", a); //(1, 1.5, true, 'a', "Hello, world!")
    println!("{:#?}",a);
    
    println!("b tuple:");
    println!("{:?}", b); 
    println!("{:#?}",b);
    
    println!("c tuple:");
    println!("{:?}", c); 
    println!("{:#?}",c);
    
    println!("d tuple:");
    println!("{:?}", d); 
    println!("{:#?}",d);
    
    println!("e tuple:");
    println!("{:?}", e); 
    println!("{:#?}",e);
    
    println!("f tuple:");
    println!("{:?}", f); 
    println!("{:#?}",f);
    
    println!("g tuple:");
    println!("{:?}", g); 
    println!("{:#?}",g);
    
    println!("h tuple:");
    println!("{:?}", h); 
    println!("{:#?}",h);

}

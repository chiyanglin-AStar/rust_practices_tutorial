fn main() {	    
    use num_bigint::{BigInt, Sign};    
    
    let mut a = BigInt::new(Sign::Plus,vec![1,0]);
    println!("{a}");
    for  i in 1..201{
        a *= i;
        println!("{i}!={a}");
    }
	println!("Hello, world!");
}

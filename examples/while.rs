fn main() {
    let mut a = 1;
    while a <= 10 {
	    println!("Current value : {}", a);
	    a += 1; //no ++ or -- in Rust
    }

    // Usage of break and continue
    let mut b = 0;
    while b < 5 {
	    if b == 0 { 
		    println!("Skip value : {}", b);
		    b += 1;
		    continue;
	    } else if b == 2 {
		    println!("Break At : {}", b);
		    break;
	    }
	    println!("Current value : {}", b);
	    b += 1;
    }

    // Outer break
    let mut c1 = 1;
    'outer_while: while c1 < 6 { //set label outer_while
	    let mut c2 = 1;
	    'inner_while: while c2 < 6 { 
		    println!("Current Value : [{}][{}]", c1, c2);
		    if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
		    c2 += 1;
	    }
	    c1 += 1;
    }
}

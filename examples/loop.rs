fn main() {
    /*
    loop {
	    println!("Loop forever!");
    }
    */

    // Usage of break and continue
    let mut a = 0;
    loop {
	    if a == 0 {
		    println!("Skip Value : {}", a);
		    a += 1;
		    continue;
	    } else if a == 2 {
		    println!("Break At : {}", a);
		    break;
	    }
	    println!("Current Value : {}", a);
	    a += 1;
    }

    // Outer break
    let mut b1 = 1;
    'outer_loop: loop { //set label outer_loop
    let mut b2 = 1;
    'inner_loop: loop {
        println!("Current Value : [{}][{}]", b1, b2);
        if b1 == 2 && b2 == 2 {
            break 'outer_loop; // kill outer_loop
        } else if b2 == 5 {
    	    break;
        }
        b2 += 1;
    }
    b1 += 1;
    }
}

fn main() {
    for a in 0..10 { //(a = 0; a <10; a++) // 0 to 10(exclusive)
        println!("Current value : {}", a);
    }

    // Usage of break and continue
    for b in 0..6 {
        if b == 0 {
            println!("Skip Value : {}", b);
            continue;
        } else if b == 2 {
            println!("Break At : {}", b);
            break;
        }
        println!("Current value : {}", b);
    }

    // Outer break
    'outer_for: for c1 in 1..6 { //set label outer_for
        'inner_for: for c2 in 1..6 {
            println!("Current Value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
        }
    }


    // Working with arrays/vectors
    let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

    for n in 0..group.len() { //group.len() = 4 -> 0..4 👎 check group.len()on each iteration
        println!("Current Person : {}", group[n]);
    }

    for person in group.iter() { //👍 group.iter() turn the array into a simple iterator
        println!("Current Person : {}", person);
    }
}

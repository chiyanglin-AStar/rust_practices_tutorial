fn main() {
    // if else  Example
    let team_size = 7;
    if team_size < 5 {
        println!("Small");
    } else if team_size < 10 {
        println!("Medium");
    } else {
        println!("Large");
    }

    // partially refactored code
    let team_size = 7;
    let team_size_in_text;
    if team_size < 5 {
        team_size_in_text = "Small";
    } else if team_size < 10 {
        team_size_in_text = "Medium";
    } else {
        team_size_in_text = "Large";
    }
    println!("Current team size : {}", team_size_in_text);

    //optimistic code
    let team_size = 7;
    let team_size_in_text = if team_size < 5 {
        "Small" //⭐️no ;
    } else if team_size < 10 {
        "Medium"
    } else {
        "Large"
    };
    println!("Current team size : {}", team_size_in_text);


    let is_below_eighteen = if team_size < 18 { true } else { false };
    
}

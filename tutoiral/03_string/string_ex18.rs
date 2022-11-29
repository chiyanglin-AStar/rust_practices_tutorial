
fn main() {
    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }

    println!("");
    
    let dog = hachiko.chars().nth(1);
    println!("{:?}, ", dog);
    // print!("{}, ", dog);
    //    = help: the trait `std::fmt::Display` is not implemented for `Option<char>`
    //    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //    = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
    //    error: aborting due to previous error
    //    For more information about this error, try `rustc --explain E0277`.
    
    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;
    print!("{}",hello_world);
    
}

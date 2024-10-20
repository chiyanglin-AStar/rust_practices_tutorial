/*

rustc --cfg some_condition custom_cfg.rs && ./custom_cfg
condition met!

*/

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

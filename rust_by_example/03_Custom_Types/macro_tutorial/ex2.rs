macro_rules! hello {
    [] => {
        println!("Hello!");
    };
    {world} => {
        println!("Hello, world!");
    };
}
 
fn main() {
    hello!();
    hello![];
    hello! {};
    hello!(world);
    hello![world];
    hello! {world};
}
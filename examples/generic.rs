// generalizing functions
//-----------------------
fn takes_anything<T>(x: T) { // x has type T, T is a generic type
}

fn takes_two_of_the_same_things<T>(x: T, y: T) { // both x and y has same type
}

fn takes_two_things<T, U>(x: T, y: U) { // multiple types
}


// generalizing structs
//---------------------
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let point_a = Point { x: 0, y: 0 }; // T is a int type
  let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type
}

// 🔎 When addding an implementation for a generic struct, the type parameters should be declared after the impl as well
// impl<T> Point<T> {


// generalizing enums
//-------------------
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 02. test_crate_hello_world/src/lib.rs
//! A Simple Hello World Crate

/// This function returns the greeting; Hello, world!
pub fn hello() -> String {
    ("Hello, world!").to_string()
}

#[cfg(test)]
mod tests {

    use super::hello;
    
    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}

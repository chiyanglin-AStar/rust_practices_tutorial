
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

#### [ch21_Testing](https://doc.rust-lang.org/rust-by-example/testing.html) 
    
    cargo new test-1

    cd test-1

    cargo test 

[ref code:](https://ithelp.ithome.com.tw/m/articles/10201066)

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 只在測試模式時編譯這個模組
    #[cfg(test)]
    mod tests {
        // 將上層的東西全部引入
        use super::*;
    
        // 標記這個是一個測試
        #[test]
        fn test_add_should_work_correctly() {
        // assert_eq! 會確定兩邊是相等的，若不是就會 panic
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(2, 3), 5);
        }
    }
    
    
    fn main() {
        println!("Hello World {}",add(10,15));
    }
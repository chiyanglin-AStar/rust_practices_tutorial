
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

#### [ch03 Custom_Types](https://doc.rust-lang.org/rust-by-example/custom_types.html) 
    
- [macro tutorial_Rust Macros: Practical Examples and Best Practices](https://earthly.dev/blog/rust-macros/)

-[Macros By Example_The Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)

-[Rust 學習之路─第十九章：巨集](https://magiclen.org/rust-macro/)

-[巨集](https://rust-lang.tw/book-tw/ch19-05-macros.html)


#### macro examples:

    //declarative_macro.rs

    macro_rules! add {
        ($a:expr)=>{
            $a
        };
        ($a:expr,$b:expr)=>{
            $a+$b
        };
        ($a:expr,$($b:tt)*)=>{
            $a+add!($($b)*)
        }
    }
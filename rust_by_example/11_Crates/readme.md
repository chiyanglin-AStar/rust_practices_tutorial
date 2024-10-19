
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

#### [ch11_Crates](https://doc.rust-lang.org/rust-by-example/crates.html) 


    $ rustc --crate-type=lib rary.rs
    $ ls lib*
    library.rlib
    
    rustc executable.rs --extern rary=library.rlib && ./executable 
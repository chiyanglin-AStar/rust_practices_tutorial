### **Command-Line Input Handling in Rust: Float, String, and Integer Data Types**

In Rust, you can handle **command-line inputs** using the `std::env` module to read arguments and then parse them into different data types such as **float**, **string**, and **integer**.

---

## **1. Basic Command-Line Input Parsing**

When running a Rust program, arguments are passed as a list of **strings**. These need to be explicitly converted into other data types using parsing functions like:

- `.parse::<T>()` where `T` is the target type (e.g., `i32`, `f64`).

---

## **2. Example: Parse Float, String, and Integer from Command-Line Arguments**

Below is an example program demonstrating how to accept and parse **integer**, **float**, and **string** command-line arguments.

### **Rust Example: Command-Line Input Parsing**

```rust
use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if sufficient arguments are passed
    if args.len() != 4 {
        println!("Usage: {} <integer> <float> <string>", args[0]);
        return;
    }

    // Parse Integer Argument
    let int_arg: i32 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: The first argument must be an integer.");
            return;
        }
    };

    // Parse Float Argument
    let float_arg: f64 = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: The second argument must be a float.");
            return;
        }
    };

    // Parse String Argument (No parsing needed for strings)
    let string_arg: String = args[3].clone();

    // Print the parsed values
    println!("Integer Argument: {}", int_arg);
    println!("Float Argument: {}", float_arg);
    println!("String Argument: {}", string_arg);
}
```

---

## **3. Explanation of the Code**

1. **Command-Line Argument Collection:**
   ```rust
   let args: Vec<String> = env::args().collect();
   ```
   - Collects arguments into a `Vec<String>`.

2. **Argument Count Check:**
   ```rust
   if args.len() != 4 {
       println!("Usage: {} <integer> <float> <string>", args[0]);
       return;
   }
   ```
   - Ensures the correct number of arguments are passed.

3. **Parsing Integer Argument:**
   ```rust
   let int_arg: i32 = match args[1].parse() {
       Ok(value) => value,
       Err(_) => {
           println!("Error: The first argument must be an integer.");
           return;
       }
   };
   ```
   - Parses the first argument as an integer (`i32`).

4. **Parsing Float Argument:**
   ```rust
   let float_arg: f64 = match args[2].parse() {
       Ok(value) => value,
       Err(_) => {
           println!("Error: The second argument must be a float.");
           return;
       }
   };
   ```
   - Parses the second argument as a float (`f64`).

5. **Handling String Argument:**
   ```rust
   let string_arg: String = args[3].clone();
   ```
   - The third argument is used directly as a string.

6. **Printing the Values:**
   ```rust
   println!("Integer Argument: {}", int_arg);
   println!("Float Argument: {}", float_arg);
   println!("String Argument: {}", string_arg);
   ```
   - Outputs the parsed values.

---

## **4. Running the Program**

Save the code as `main.rs`, then build and run the program:

```bash
cargo run -- 42 3.14 HelloWorld
```

**Expected Output:**
```
Integer Argument: 42
Float Argument: 3.14
String Argument: HelloWorld
```

---

## **5. Error Handling Examples**

If incorrect types are passed:

```bash
cargo run -- abc 3.14 Hello
```

**Output:**
```
Error: The first argument must be an integer.
```

If insufficient arguments are passed:

```bash
cargo run -- 42
```

**Output:**
```
Usage: target/debug/your_program <integer> <float> <string>
```

---

## **6. Key Takeaways**
1. **Command-Line Arguments:** Use `std::env::args()` to collect arguments.
2. **Parsing Data Types:** Use `.parse::<T>()` for integer and float parsing.
3. **Error Handling:** Use `match` or `unwrap_or_else` for safe parsing.
4. **String Handling:** Strings are directly accessible without parsing.

---

If you'd like further clarification or an advanced example (e.g., handling multiple inputs dynamically), let me know! 🚀
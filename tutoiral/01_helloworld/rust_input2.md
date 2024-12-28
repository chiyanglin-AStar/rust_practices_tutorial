### **Command-Line Input in Rust (Like `scanf` in C)**

In **C language**, `scanf` is commonly used to get user input interactively from the **standard input (stdin)**. In Rust, similar behavior can be achieved using the **`std::io`** module.

---

## **1. Introduction to Command-Line Input in Rust (Interactive Mode)**

- **Rust Equivalent to `scanf`:**
   - Use `std::io::stdin()` for user input.
   - Parse the input into **integer**, **float**, or **string** formats.

- **Key Methods:**
   - `std::io::stdin().read_line(&mut buffer)`
   - `.trim()` to remove newline characters.
   - `.parse::<T>()` to parse input into a specific type (`i32`, `f64`, `String`).

---

## **2. Example: Interactive User Input (Integer, Float, String)**

```rust
use std::io;

fn main() {
    // Integer Input
    let mut int_input = String::new();
    println!("Enter an integer:");
    io::stdin().read_line(&mut int_input).expect("Failed to read line");
    let int_value: i32 = match int_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid integer input!");
            return;
        }
    };

    // Float Input
    let mut float_input = String::new();
    println!("Enter a float:");
    io::stdin().read_line(&mut float_input).expect("Failed to read line");
    let float_value: f64 = match float_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid float input!");
            return;
        }
    };

    // String Input
    let mut string_input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut string_input).expect("Failed to read line");
    let string_value = string_input.trim(); // Strings do not need parsing

    // Output the values
    println!("\nYou entered:");
    println!("Integer: {}", int_value);
    println!("Float: {}", float_value);
    println!("String: {}", string_value);
}
```

---

## **3. Explanation of the Code**

1. **Reading Integer Input:**
   - `read_line` captures input as a string.
   - `.trim()` removes newline characters.
   - `.parse::<i32>()` converts the string to an `i32` integer.

2. **Reading Float Input:**
   - Input is parsed into an `f64` float after trimming.

3. **Reading String Input:**
   - The input string is trimmed and directly used.

4. **Error Handling:**
   - Each parsing operation uses `match` for error handling.
   - Invalid inputs produce error messages.

---

## **4. Example Input/Output**

### **Input:**
```text
Enter an integer:
42
Enter a float:
3.14
Enter a string:
Hello, Rust!
```

### **Output:**
```text
You entered:
Integer: 42
Float: 3.14
String: Hello, Rust!
```

### **Invalid Input Example:**
```text
Enter an integer:
abc
```

**Output:**
```text
Invalid integer input!
```

---

## **5. Shortcut with `expect` (Quick Parsing Without Match)**

If you're confident the input will always be valid:

```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let int_value: i32 = input.trim().parse().expect("Invalid integer input");

    input.clear(); // Clear buffer for reuse
    println!("Enter a float:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let float_value: f64 = input.trim().parse().expect("Invalid float input");

    input.clear();
    println!("Enter a string:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let string_value = input.trim();

    println!("\nYou entered:");
    println!("Integer: {}", int_value);
    println!("Float: {}", float_value);
    println!("String: {}", string_value);
}
```

**Pros:** Cleaner code.  
**Cons:** Program will panic (`expect`) on invalid input.

---

## **6. Differences Between Rust and C `scanf`**

| **Feature** | **C (`scanf`)** | **Rust (`read_line`)** |
|-------------|------------------|-----------------------|
| **Syntax** | `scanf("%d", &num)` | `stdin().read_line()` |
| **Parsing** | Automatic based on format specifier | Manual with `.parse()` |
| **Error Handling** | Limited error detection | Match or expect for handling errors |
| **Buffer Clearing** | Implicit | Explicit `.clear()` required |

---

## **7. Advanced Parsing Example (Multiple Inputs in One Line)**

Users can input multiple values on the same line, and Rust can parse them:

```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer, a float, and a string (separated by spaces):");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut inputs = input.trim().split_whitespace();
    let int_value: i32 = inputs.next().unwrap().parse().expect("Invalid integer");
    let float_value: f64 = inputs.next().unwrap().parse().expect("Invalid float");
    let string_value = inputs.next().unwrap_or("Default");

    println!("\nYou entered:");
    println!("Integer: {}", int_value);
    println!("Float: {}", float_value);
    println!("String: {}", string_value);
}
```

### **Input Example:**
```text
42 3.14 HelloRust
```

### **Output:**
```text
Integer: 42
Float: 3.14
String: HelloRust
```

---

## **8. Key Takeaways**
1. **Read from stdin:** Use `io::stdin().read_line(&mut buffer)`.
2. **Trim input:** Use `.trim()` to remove newline characters.
3. **Parse into types:** Use `.parse::<T>()` to convert string input into integers or floats.
4. **Error Handling:** Use `match` for safe parsing or `expect` for simplicity.
5. **Multiple Inputs:** Split input with `.split_whitespace()`.

---

## **9. Further Improvements**
- Use **custom structs** to encapsulate inputs.
- Add **looping mechanisms** to handle retry on invalid inputs.

Would you like to see an example with **custom data structures** or **interactive prompts** for more complex parsing? 🚀
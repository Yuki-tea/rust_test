# Rust Basics Reference 🦀

## Variables
By default, `let` is like `const` in JS. You must explicitly type `mut` to allow changes.
```rust
let x = 5;
// x = 6; // ❌ COMPILER ERROR! 

let mut y = 5;
y = 6;    // ✅ Works! 
```

## Conditionals
No parentheses are needed around the condition. In Rust, `if` is an expression, meaning it can return a value (replacing the Ternary Operator). Notice there are NO semicolons inside the blocks when returning a value!
```rust
let age = 20;

if age >= 18 {
    println!("Adult");
} else {
    println!("Minor");
}

// 🤯 Replacing the Ternary Operator:
let status = if age >= 18 { "Adult" } else { "Minor" };
```

## Loops

### While
```rust
let mut n = 3;
while n > 0 {
    println!("{}", n);
    n -= 1;
}
```

### For
```rust
// Loops from 0 up to 9 (exclusive)
for i in 0..10 {
    println!("Number: {}", i);
}

// Loops from 1 to 5 (inclusive)
for i in 1..=5 {
    println!("Count: {}", i);
}

let web_frameworks = vec!["Next.js", "NestJS", "Express"];
for framework in web_frameworks {
    println!("I use {}", framework);
}
```

### Loop (= while true {})
```rust
let mut counter = 0;

// This loop will run forever until we explicitly break it
let result = loop {
    counter += 1;

    if counter == 10 {
        // Break out of the loop, AND return the number 42 into the `result` variable!
        break 42;
    }
};

println!("The loop returned: {}", result); // Prints 42
```

## Data Structures

### Arrays (fixed size, fast)
```rust
// Type signature: [type; size]
let numbers: [i32; 3] = [1, 2, 3];
println!("First number: {}", numbers[0]);
```

### Vectors (dynamic size)
```rust
let mut frameworks = vec!["React", "NestJS"];
frameworks.push("Next.js"); // Grows dynamically!
```

### HashMap (key-value)
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 85);

// Reading a value (returns an Option because it might be null/missing!)
let alice_score = scores.get("Alice");
```

### HashSet
```rust
use std::collections::HashSet;

let mut seen_ips = HashSet::new();
seen_ips.insert("192.168.1.1");
seen_ips.insert("192.168.1.1"); // This gets safely ignored!
```

## OOP (Structs, Impl, and Traits)
Rust separates data (`struct`) from behavior (`impl`). Interfaces are called `trait`.

```rust
// 1. Define the Interface (The Trait)
trait Formattable {
    // Any struct that implements this trait MUST have a `format_data` method
    fn format_data(&self) -> String;
}

// 2. Define our Structs (The Data)
struct Article {
    title: String,
    author: String,
}

struct User {
    username: String,
    is_active: bool,
}

// 3. Implement standard methods for a Struct
impl User {
    fn new(username: &str) -> User {
        User {
            username: String::from(username),
            is_active: true,
        }
    }

    fn deactivate(&mut self) {
        self.is_active = false;
    }
}

// 4. Implement the Trait for our Structs
impl Formattable for Article {
    fn format_data(&self) -> String {
        format!("Article: '{}' by {}", self.title, self.author)
    }
}

impl Formattable for User {
    fn format_data(&self) -> String {
        format!("User Profile: @{}", self.username)
    }
}

fn main() {
    let my_article = Article { title: String::from("Rust Basics"), author: String::from("Yuki") };
    let mut my_user = User::new("yuki_dev");

    my_user.deactivate(); // Calling the standard method

    // Both structs can now use the shared trait behavior!
    println!("{}", my_article.format_data());
    println!("{}", my_user.format_data());
}
```

## Match (Switch statement in Rust)
Just like `if`, `match` is an expression and can return a value! It also forces exhaustive checking.

```rust
let status_code = 404;

let message = match status_code {
    200 => "OK: Request successful",
    404 => "Not Found: Page is missing",
    500 => "Server Error: Something broke",

    // The `_` is the default "catch-all" (like `default:` in a switch)
    // If you delete this line, Rust will throw a compiler error because
    // it knows you haven't covered every other possible integer!
    _ => "Unknown Error",
};

println!("Server says: {}", message);
```

## Error Handling & Null Safety (`Option` and `Result`)
Rust does not have `null` or exceptions (like `try/catch`). It uses Enums to physically force you to handle missing data or errors before the code is allowed to compile.

### `Option<T>`: The Null Killer
Used when a value might be missing. It returns either `Some(data)` or `None`.
```rust
fn find_user(id: i32) -> Option<String> {
    if id == 1 { 
        Some(String::from("Yuki")) 
    } else { 
        None 
    }
}

// You MUST handle both cases using `match` to unwrap the box!
match find_user(1) {
    Some(name) => println!("Found user: {}", name),
    None => println!("No user found."),
}
```

### `Result<T, E>`: The Exception Killer
Used when an operation might crash or fail (like reading a file or parsing data). It returns either `Ok(data)` or `Err(error)`.
```rust
// "hello" cannot be parsed into an integer, so this will return an Err!
let attempt = "hello".parse::<i32>();

match attempt {
    Ok(number) => println!("Parsed: {}", number),
    Err(error) => println!("Failed: {}", error),
}
```

### The `?` Operator (Automatic Error Return)
If you are writing a function that returns a `Result`, you don't need to write huge `match` blocks. Just add `?` to the end of a risky function call. 

If the call succeeds, it extracts the `Ok` data. If it fails, it instantly stops your function and returns the `Err` to whatever called it!
```rust
use std::fs::File;
use std::io::Read;

// The function MUST return a Result to use the `?` operator inside it!
fn read_config() -> Result<String, std::io::Error> {
    let mut file = File::open("config.txt")?; // Fails? Return Err instantly.
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;      // Fails? Return Err instantly.
    
    Ok(contents)                              // Success! Return the data.
}
```

### The `.unwrap()` Escape Hatch
If you are 100% sure a function won't fail (or you *want* it to intentionally crash the entire program if it does fail), use `.unwrap()` to skip the `match` and extract the data directly.
```rust
// We know "42" is a valid number, so we bypass the Result and unwrap it.
let number: i32 = "42".parse().unwrap(); 
```

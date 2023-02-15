<a name="readme-top"></a>

# ⚙️ ***Rust*** - `Basics`

This is a quick introduction to the Rust programming language. It is not a complete reference, but rather a quick overview of the most important concepts. For a more complete reference, see the [Rust book](https://doc.rust-lang.org/book/).


### Variables

Variables are immutable by default. To make a variable mutable, use the `mut` keyword.
```rust
    let x = 5; // immutable
    let mut y = 5; // mutable
```

### Data types

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.
```rust
    let x = 5; // i32
    let y = 5.0; // f64
```

### Functions

Functions are declared using the `fn` keyword. The return type is specified after an arrow (`->`).

```rust
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
```

### Control flow

Rust has `if` expressions.

```rust
    if x < 5 {
        println!("x is less than five!");
    } else {
        println!("x is five or greater!");
    }
```

Rust has `loop` expressions.

```rust
    loop {
        println!("again!");
    }
```

Rust has `while` expressions.
```rust
    while x < 5 {
        println!("x is less than five!");
    }
```

Rust has `for` expressions.

```rust
    for x in 0..10 {
        println!("x is {}", x);
    }
```

### Ownership

Each value in Rust has a variable that’s called its owner. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.

```rust
    fn main() {
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid
```

### References and borrowing

References allow you to refer to some value without taking ownership of it. References are immutable by default, and they must always be valid.

```rust
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    }
```

### Slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

```rust
    fn main() {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }
```

### Structs

Structs are used to create custom data types.

```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn main() {
        let mut user1 = User {
            email: String::from("[...]"),
            username: String::from("[...]")
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("[...]");
    }
```

### Enums

Enums are used to create a type which may be one of a few different variants.
```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    fn main() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("[...]),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("[...]),
        };
    }
```

### Methods

Methods are similar to functions: they’re declared with the `fn` keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, and their first parameter is always `self`, which represents the instance of the struct the method is being called on.
```rust
    impl User {
        fn email(&self) -> &String {
            &self.email
        }

        fn username(&self) -> &String {
            &self.username
        }

        fn sign_in_count(&self) -> &u64 {
            &self.sign_in_count
        }

        fn active(&self) -> &bool {
            &self.active
        }
    }
```

### Traits

Traits are similar to a feature often called interfaces in other languages, although with some differences.

```rust
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        headline: String,
        location: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
```

<br>

<p align="right"><a href="#readme-top">🔝</a></p>

[↩️](README.md)
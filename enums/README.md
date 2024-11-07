# Enums
- Enums allow you to define a type by enumerating its possible variants
- Enums give you a way of saying a value is one of a possible set of values

```Rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- We can attach data to an enum directly,
```Rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
-  the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
- There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.

Example
```Rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
This enum has four variants with different types:
- `Quit` has no data associated with it at all.
- `Move` has named fields, like a struct does.
- `Write` includes a single `String`.
- `ChangeColor` includes three `i32` values.

- There is one more similarity between enums and structs: just as we’re able to define methods on structs using `impl`, we’re also able to define methods on enums.

```Rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

# The `Option` Enum
- The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.
- Defined in standard library as follows:
```Rust
enum Option<T> {
    None,
    Some(T),
}
```

- The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter
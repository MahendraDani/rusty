# The Slice Type
Slices let you reference a contiguous sequence of elements in a [collection](https://doc.rust-lang.org/book/ch08-00-common-collections.html) rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

## String Slices
A string slice is a reference to part of a `String`.

```Rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

With Rust’s `..` range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
```Rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```
By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
```Rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```
You can also drop both values to take a slice of the entire string. So these are equal:
```Rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

## String Literals as slices

```Rust
let s = "Hello, world!";
```
The type of `s` here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an **_immutable reference_**.

Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality:
```Rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```


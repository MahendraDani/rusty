# References and Borrowing
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.

 Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

 ```Rust
 fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

![Reference](/public/images/ch4/reference.png)

> NOTE : The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*.` 

When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
We call the action of creating a reference **_borrowing_**.

So, what happens if we try to modify something we’re borrowing? Try the code in Listing 4-6. Spoiler alert: **it doesn’t work**!

```Rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // ERROR
}
```

> NOTE: Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

# Mutable References
```Rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // ERROR

println!("{}, {}", r1, r2); 
```

The benefit of having this restriction is that Rust can prevent _data races_ at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```Rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

We also cannot have a mutable reference while we have an immutable one to the same value.
```Rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
```Rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```
The scopes of the immutable references `r1` and `r2` end after the `println!`

# Dangling references
Dangling Reference : a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.

In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
```Rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s   // ERROR
}
```

Error : this function's return type contains a borrowed value, but there is no value
for it to be borrowed from

```Rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

The solution here is to return the `String` directly:
```Rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

# The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
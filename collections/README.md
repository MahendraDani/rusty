# Creating a new vector
1. By using `new` keyword:
```
let v : Vec<i32> = Vec::new()
```

2. By using `vec!` macro
```
let v = vec![1,5,7,9];
```

# Reading elements
1. By using indexing or by using `get` method
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

# Iterating over values in a vector
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

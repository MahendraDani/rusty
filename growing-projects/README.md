# Crates
- Crates can be either binary crates or library crates.
- Binary creates have src/main.rs file and the have a main function which can be executed.
- Library creates have src/main.rs doesn't contain main function.

# Packages
- A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.
- A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
- A package can contain as many binary crates as you like, but at most only one library crate.

# Modules : Cheatsheet
1. Declaring Modules : a module named "garden" can be checked by the compiler at
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file src/garden.rs
  - In the file src/garden/mod.rs

2. Declaring sub-modules:
  - Inline,
  - In the file, src/vegetables.rs
  - In the file, src/vegetables/mod.rs
3. Paths to Code in Modules : Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code.
ex- `create::garden::fruits::apple`

4. Private vs Public : Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of mod.

# Grouping Related Modules Together
- Modules let us organize code within a crate for readability and easy reuse.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

# Paths
There are two forms of paths:
1. Absolute Path : An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
2. Relative Path : A relative path starts from the current module and uses self, super, or an identifier in the current module. 

> NOTE : Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. 

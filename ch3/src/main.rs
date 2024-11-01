fn main() {
    let x = 10;
    println!("Value of x: {}",x); // 10

    // variable shadowing
    let x = String::from("Hello World"); // Hello World
    println!("Value of x: {}",x);
}

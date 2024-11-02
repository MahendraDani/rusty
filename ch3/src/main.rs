fn main() {
    let x = 10;
    println!("Value of x: {}",x); // 10

    // variable shadowing
    let x = String::from("Hello World"); // Hello World
    println!("Value of x: {}",x);

    // Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    
    // pattern matching (similar to destructing in js)
    let (a,b,c) = x;
    println!("{}, {}, {}",five_hundred,six_point_four,one);
    println!("{}, {}, {}",a,b,c);

    // Arrays
    let arr = ["Orange", "Apples","Mango"];
    for i in 0..arr.len(){
        println!("{}",arr[i]);
    }
}

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}",s);

    let s1 = String::from("Hi");
    let s2 = s1;
    println!("{}",s1);
}

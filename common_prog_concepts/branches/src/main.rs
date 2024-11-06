fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let flag = false;
    let ans = if flag {10} else {100};
    println!("{}",ans);
}
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }


enum Message {
    Quit,
    Write(String),
    Read
}

impl Message {
    fn write(s :String){
        println!("Writing : {}",s);
    }
}
fn main() {
    // let home = IpAddrKind::V4(String::from("127.0.0.1"));

    // let loopback = IpAddrKind::V6(String::from("::1"));
    println!("Hello, world!");
    let m = Message::Write(String::from("value"));
}

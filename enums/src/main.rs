enum IPAddrKind {
    V4,
    V6,
}

enum Message {
    Write(String),
}
impl Message {
    fn call(&self) {
        println!("Message");
    }
}
fn main() {
    let num = Some(5);
    let absent_number: Option<i32> = None;
    println!("{absent_number:?}");
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("Enums");
}

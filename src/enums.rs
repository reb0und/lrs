enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    //let four = IpAddr::V4;
    //let six = IpAddr::V6;
    //
    //route(IpAddr::V4);
    //route(IpAddr::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hi"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddr) {}

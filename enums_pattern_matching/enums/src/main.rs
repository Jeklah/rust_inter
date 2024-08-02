enum IpaddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpaddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn route(ip_type: IpaddrKind) {}

fn main() {
    let four = IpaddrKind::V4;
    let six = IpaddrKind::V6;
    let home = IpAddr {
        kind: IpaddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpaddrKind::V6,
        address: String::from("::1"),
    };

    route(IpaddrKind::V4);
    route(IpaddrKind::V6);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;
}

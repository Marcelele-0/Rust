// enums
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}


// we could use struct to define the same thing
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// but its not as clear as enum

// now method can be defined for enum
impl Message {
    fn some_method() {
        println!("some method")
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;

    let local_host = IpAddrKind::V4(127, 0, 0, 1);
}

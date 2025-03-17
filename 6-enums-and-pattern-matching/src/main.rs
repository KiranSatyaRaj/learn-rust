enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// each variant can have their
// own associated types
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpV4Addr {}

struct IpV6Addr {}

enum IpAddr3 {
    V4(IpV4Addr),
    V6(IpV6Addr),
}


// another example
enum Message {
    Quit,                       // has no associated data
    Move {x: i32, y: i32},      // named fields like structs
    Write(String),              // single String
    ChangeColor(i32, i32, i32), // include 3 i32 values
}

// we can implement methods for enums
// similar to a struct
impl Message {
    fn call(&self) {
        // method body
    }
}

// stdlib feature for
// a value being present or absent
// it's included a prelude and
// can be used without Option:: prefix
// enum Option<T> { // here T is a generic type parameter
//     None,
//     Some(T),
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option
    // it forces us to check if there's a possibility
    // of a None value being present
    // which not the case with T as Option<T> and T
    // are different types
    let some_number = Some(32);  // Type: Option<i32>
    let some_char = Some('a');   // Type: Option<char>

    let absent_num = None;       // Type: Option<None>

    let x: Option<i32> = Some(245);
    let y: Option<i32> = Some(4);
    // let sum = x + y;        // this won't compile
}

fn route(ip_kind: IpAddrKind) {}
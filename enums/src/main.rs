mod if_let;
mod pattern_matching;

// wanting to store IP adresses is a common task, and the standard
// library already has a definition that can be used
// https://doc.rust-lang.org/std/net/enum.IpAddr.html
struct Ipv4Addr {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}
struct Ipv6Addr {
    address: String,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
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
    let four = IpAddr::V4(Ipv4Addr {
        a: 127,
        b: 0,
        c: 0,
        d: 1,
    });
    let six = IpAddr::V6(Ipv6Addr {
        address: String::from("::1"),
    });

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();

    // the Option enum, as defined by the standard library:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // the `<T>` is a generic parameter

    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    // WILL NOT COMPILE
    // i32 and Option<i32> are different types
    // let x = 5;
    // let sum = x + _some_number;

    if_let::main();
    pattern_matching::main();
}

fn route(_ip_kind: IpAddr) {}

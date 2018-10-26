#[derive(Debug)]
// enums are useful when we know a variable can only have a fixed set of values.
enum IpAddrKind {
    V4,
    V6,
}

// we can pair enums with struct to pair data related to enum value
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

// Without pairing with struct we can add values to enum values
#[derive(Debug)]
enum IpAddrKindValued {
    V4(u8, u8, u8, u8), // unlike struct each field can have different type of types and amount associated data
    V6(String),
}

// here we have an enum that can be either a unit, struct, string, or a tuple.
// we can emulate this using four different struct.
// incase of function arguments where we want to take a Message which can be any of the four values
// in that scenario, we can only use enums not structs.
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

// similar to structs we can implement methods on enums
impl Message {
    fn call(&self) {
        println!("Called the message");
    }

    fn associated_fn() {
        println!("Called an associated_fn");
    }
}


fn main() {
    // Here we created two instances of IpAddrKind
    let i4 = IpAddrKind::V4;
    let i6 = IpAddrKind::V6;

    check_ip(i4);

    let ip = IpAddr {
        kind: i6,
        address: String::from("::1")
    };

    println!("The kind is {:?} and the address is {}", ip.kind, ip.address);

    let i4a = IpAddrKindValued::V4(0,0,0,0);
    let i6a = IpAddrKindValued::V6(String::from("::1"));
    println!("i4: {:?} | i6a: {:?}", i4a, i6a);

    let m = Message::Write(String::from("Hoi"));
    m.call();
    Message::associated_fn();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // we can't add i8 and Option<i8> because we need to check if y has a value or not. this can be
    // done using match operator
    // println!("{}", x + y);

    let x1: i8 = 5;
    let y1: Option<i8> = None;
    // println!("{}", x1+y1);
    // like the previous case we can't add i8 and Option<i8> becaue we need to get the value from
    // Option<T> like the last two case it can be 5 or None, so we explicitly check it.

}

// Here we have a function that can take values of either type IpAddrKind::V4 or IpAddrKind::V6
fn check_ip(ip: IpAddrKind) {
    println!("{:?}", ip);
}

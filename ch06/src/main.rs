#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}


fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Move{ x:3, y:2 };
    let m2 = Message::Write(String::from("hello"));
    m.call();
    m2.call();
    println!("Hello, world!");

    let sum = calc_sum();
    println!("sum {:?}", sum)
}

fn calc_sum() -> i8 {
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // Option is either None or Some

    match y {
        None => 0,
        Some(i) => x + i,
    }
}

fn route(ip_type: IpAddr) { 
    println!("{:?}", ip_type)
}
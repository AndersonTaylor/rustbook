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
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Move { x: 3, y: 2 };
    let m2 = Message::Write(String::from("hello"));
    m.call();
    m2.call();
    println!("Hello, world!");


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let sum = calc_sum();
    println!("sum {:?}", sum);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // if doesn't match => do nothing 
    }
    println!("end");

    println!();

    let mut xyz_coin = Coin::Penny;
    let mut non_quarters = 0;
    println!("{}", value_in_cents(xyz_coin));
    if let Coin::Quarter(state) = xyz_coin {
        println!("Not counting State quarter from {:?}!", state);
    } else {
        non_quarters += 1;
    }

    xyz_coin = Coin::Quarter(UsState::Kentucky);
    println!("{}", value_in_cents(xyz_coin));

    if let Coin::Quarter(state) = xyz_coin {
        println!("Not counting State quarter from {:?}!", state);
    } else {
        non_quarters += 1;
    }
    println!("non quarters {}", non_quarters);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
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

#[derive(Debug, Copy, Clone)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Kentucky,
    Texas,
}

#[derive(Copy, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


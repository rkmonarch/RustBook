fn main() {
      let localhost = IpAddrKind::V4(12,0,0,1);
      let quit_program = Message::Move { X:20, y: 20};
     println!("Your Ip Address is: {:?}", localhost);
    Optional_value();
    value_in_cents(Coin::Quarter((UsState::Alaska)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("You have these two numbers, {:?} and {:?}",six,none);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("Three");
    }
}

fn route(ip_kind: IpAddr) {}

fn Optional_value() {
    let x = 20;
    let y: Option<i32> = None;
    let sum = x + y.unwrap_or(0);
    println!("The sum is: {}", sum);
}
enum Coin {
    Penny,
    Nikkel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nikkel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(i:Option<i32>) -> Option<i32>{
    match i {
        Some(i) => Some(i + 1),
        _ => None,
}}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { X: i32, y: i32 },
    Writ(String),
    ChangeColor(i32, i32, i32),
}

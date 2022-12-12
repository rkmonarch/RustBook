fn main() {
//   let localhost = IpAddrKind::V4(12,0,0,1);
//   let quit_program = Message::Move { X:20, y: 20};
//  println!("Your Ip Address is: {:?}", localhost);
Optional_value();
}

fn route(ip_kind:IpAddr){

}

fn Optional_value(){
    let x = 20;
    let y : Option<i32>  = None;
    let sum = x + y.unwrap_or(0);
    println!("The sum is: {}", sum);

}

struct IpAddr{
    kind:IpAddrKind,
    address:String
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
#[derive(Debug)]
enum Message{
    Quit,
    Move{X:i32, y:i32},
    Writ(String),
    ChangeColor(i32,i32,i32)
}
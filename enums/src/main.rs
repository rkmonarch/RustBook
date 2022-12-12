fn main() {
  let localhost = IpAddrKind::V4(12,0,0,1);
  println!("Your Ip Address is: {:?}", localhost);
}

fn route(ip_kind:IpAddr){

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

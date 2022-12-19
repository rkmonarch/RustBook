fn main() {
  let a = [10,20,30];
  let mut v:Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  let v2 = vec![1,2,3];

  let third = &v2[2];
//   println!("third element: {:?}", third);
match v2.get(3){
    Some(x) => println!("second element: {:?}", x),
    None => println!("there is no third element"),
}
}

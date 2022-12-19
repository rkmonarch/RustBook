use std::collections::HashMap;

fn main() {
//   let a = [10,20,30];
//   let mut v:Vec<i32> = Vec::new();
//   v.push(1);
//   v.push(2);
//   v.push(3);
//   let v2 = vec![1,2,3];

//   let third = &v2[2];
// //   println!("third element: {:?}", third);
// match v2.get(3){
//     Some(x) => println!("second element: {:?}", x),
//     None => println!("there is no third element"),
// }
hashing();
hashMaps();

}
pub fn hashing(){
  let blue = String::from("Blue");
  let green = String::from("Green");
  let mut scores = HashMap::new();

  scores.insert(blue, 10);
  scores.insert(green, 20);  
}

pub fn hashMaps(){
  let text = "Hello World wonderful World";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count+=1;
  }

  println!("{:?}", map);

  }

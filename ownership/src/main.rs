fn main() {
  
// copyvalues();
// ownerships();
  get_length(); //borrowing and reference

 let mut s = String::from("hello world");
 let s2 = "Hello world";
 let word = first_word(s2);
 s.clear(); 
 println!("{}", word)

 // whenever we need to retrun or get a slice of the string we use [start..end]
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); //convert array to bytes
    for (i, &item) in bytes.iter().enumerate() {
if item == b' ' //check for the space 

 {
    return &s[0..i]; //return the string before the space
}   
}
&s[..] //return the whole string
}
fn copyvalues(){

    copyandownership();

    let s = String::from("Takes String");
    takesownership(s);

   // println!("Value of s is{}", s); //This will give error

   let x = 10;
   copyvalue(x);
   println!("Value of x is {}", x);
}

fn ownerships(){
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_give_back(s2);
 
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello String");
some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn copyandownership(){
    let x =5;
    let y = x; //copying
    let s1 = String::from("Hello Rust!");
    let s2 = s1.clone();
 
    println!("{}, world!", s1);
}

fn takesownership(s: String)  {
println!("{}, taking ownership",s);
}

fn copyvalue(some_integer : i32){

    println!("Value of some_integer: {}",some_integer);
}

fn get_length(){
    let s1= String::from("hello world");
    let len = calculate_length(&s1);
    println!("length of {}, is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}
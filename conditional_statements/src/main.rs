fn main() {

    // Line_comments
    /*
    block comment
    */
   var_type();
  simple_loop();
  while_loop();
  for_loop();
 let result = my_function(10,20);
 println!("result: {}", result);
}

fn var_type(){
    let x   = 1;
    println!("varibale is:{}", x);
    //shadow of varibale
    let x = "Six";
    println!("varibale is:{}", x);

    let tup = ("this is string",100);
    let (channel, subCount) = tup;
    let subCount = tup.1;

    let error_codes = [200,404,500];
    let not_found = error_codes[1];
    let x = error_codes[1]; // this will give error cause length is 2
    let byte = [0; 8];
}

fn my_function(x:i32, y:i32) -> i32{
println!("Value of x is: {}", x);
println!("Value of y is: {}", y);
return x + y;
}

fn simple_loop() {
    let mut counter = 0;
  let result = loop{
    counter += 1;
    if counter == 10{
        break counter;
    }
  };
  println!("result is:{}", result);
}

fn while_loop() {
    let mut counter = 3;
    while counter != 0 {
        println!("counter is {}", counter);
        counter -= 1;
    };
    println!("Lifted")
}

fn for_loop() {
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("element is {}", element);
    }
}
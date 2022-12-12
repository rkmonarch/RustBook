fn main() {
  create_user();

// let width1 = 20;
// let height1 = 30;
// println!("Result is {}", area(width1, height1))
//instead passing two different varibale we pass one single variable

let rect = Rectangle { height: 30, width: 50 };
let rect1 = Rectangle{
    height:20,
    width:40
};
let rect2 = Rectangle{
    height:40,
    width:50
};

let rect3 = Rectangle::square(25);
println!("rect can hold rect1 is {}",rect.can_hold(&rect1));
println!("rect can hold rect2 is {}",rect.can_hold(&rect2));


println!("rect: {:#?}", rect);
println!("Result is {}", rect.area());

}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl  Rectangle {
    fn area(&self) -> u32{
self.height*self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.height >= other.height && self.width >= other.width
    }
}

impl  Rectangle{
    fn square(size:u32) -> Rectangle{
        Rectangle { height:size, width: size }
    }
}
// fn area(rectangle: &Rectangle) ->u32{
//     // dimesions.0 * dimesions.1
//     rectangle.height * rectangle.width

// } //we can use methods instead of functions

struct color(i32, i32, i32);
struct point(i32, i32, i32); //even though point and color are same both reference to the different struct members

fn create_user(){
     let mut  user1 = User{
    username:String::from("John"),
    email: String::from("EMail@example.com"),
    sign_in_count:1,
    active: false,
   };

   let name = user1.username;
   user1.username = String::from("John Cena");
   user1.email = String::from("new@example.com");
   user1.sign_in_count = 2;
   user1.active = true;

   let user2 = build_user(String::from("newuser@gmail.com"), String::from("Rahul"));
   let user3 = User{
username: String::from("Third User"),
email: String::from("EMail@example.com"),
..user2 //this will take values from user2.active and user2.sign_in_count
   };
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String)-> User {
    User{
        username,
        email,
    active:true,
    sign_in_count:1
    }
}
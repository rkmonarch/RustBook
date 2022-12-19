mod front_of_house;

// pub fn eat_at_restaurant(){
//     //Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     //Relative path
//     front_of_house::hosting::add_to_waitlist();
// }


mod back_of_house;

pub fn eat_at_restaurant(){
let mut meal = back_of_house::Breakfast::summer("Rahul");
meal.toast = String::from("Hello");
// let mut season2 = back_of_house::Breakfast{
//     toast: String::from("Hello"),
//     seasonal_fruit: String::from("Apple"), //private we can't create directly here
// };
}

mod front_of_house_2;

use crate::front_of_house_2::hosting;

pub fn eat_at_restaurant_2(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
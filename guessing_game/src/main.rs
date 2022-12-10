use rand::Rng;
use std::{cmp::Ordering, io};
use colored::*;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(0..101);

    println!("the secret number is: {}", secret_number);
    loop {
        println!("Please enter your input!");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You have guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","too small".red()),
            Ordering::Greater => println!("{}","too big".red()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            },
        }
    }
}

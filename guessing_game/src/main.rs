use rand::Rng;
use std::{cmp::Ordering, io};

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
        let guess: u32 = guess.trim().parse().expect("Please type a numberr");
        println!("You have guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}

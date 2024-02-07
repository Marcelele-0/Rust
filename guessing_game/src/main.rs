use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}.", secret_number);

    loop {
                println!("Please input your guess.");
        let mut users_guess = String::new();

        io::stdin()
            .read_line(&mut users_guess)
            .expect("Failed to read line");

        let users_guess: u32 = match users_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guessed: {}", users_guess);

        match users_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }        

}

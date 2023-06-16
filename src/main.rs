use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Welcome to the guessing game");
    println!("Enter any character to quit the game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Input your guess");

        io::stdin().read_line(&mut guess).expect("Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You decided to quit");
                break;
            }
        };

        println!("You guess {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
            cmp::Ordering::Less => println!("Too small"),
            cmp::Ordering::Greater => println!("Too big"),
        }
    }
}

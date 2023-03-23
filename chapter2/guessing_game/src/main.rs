use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("I'm thinking of a number between 0 and 100...");

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading your input!");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("higher..."),
            Ordering::Greater => println!("lower..."),
            Ordering::Equal => {
                println!("You guessed it! The number was {}", secret_number);
                break;
            }
        }
    }
}

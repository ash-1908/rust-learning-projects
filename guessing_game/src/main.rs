use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game !");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Enter your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error while stdin");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match random_number.cmp(&guess) {
            Ordering::Greater => {
                println!("Too small");
            }
            Ordering::Less => {
                println!("Too big");
            }
            Ordering::Equal => {
                println!("Correct !");
                break;
            }
        }
    }
}

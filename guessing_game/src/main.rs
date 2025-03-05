use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let mut counter = 0;
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");
        counter += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You took {counter} guesses to guess the number.");
                break;
            }
        }
    }
}

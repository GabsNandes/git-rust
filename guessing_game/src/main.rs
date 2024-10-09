use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses = 7;

    let mut win= false;

    while guesses >= 0 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}, you have {guesses} guesses remaining");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guesses -=1;

            },
            Ordering::Greater => {
                println!("Too big!");
                guesses -=1;
            },
            Ordering::Equal => {
                println!("You win!");
                win = true;
                break;
            }
        }
    }

    if !win {
       println!("You loose, the number was {secret_number}") ;
    }
}
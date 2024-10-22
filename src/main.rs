use std::io::{self, Write};

use rand::Rng;

fn main() {
    let mut stdout = io::stdout();

    let stdin = io::stdin();

    let number = rand::thread_rng().gen_range(0..100);

    let mut guess: i32;

    let mut attempts = 5;

    loop {
        print!("Enter your guess: ");

        stdout.flush().unwrap();
        
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        guess = input.trim().parse::<i32>().unwrap();

        if guess == number {
            println!("You win!");

            break;
        }

        attempts -= 1;

        if attempts == 0 {
            println!("You lose!");

            break;
        }

        if guess < number {
            println!("Too small!");
        } else {
            println!("Too big!");
        }
    }
}

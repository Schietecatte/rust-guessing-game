use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=================================================");
    println!("WELCOME!! Welcome to the game: 'Guess the number'");
    println!("...");
    println!("Let's start! 🥵");
    println!("=================================================");
    println!();

    let max_number: u32;

    loop {
        let mut max_number_input: String = String::new();

        println!("What should be the maximum number?...");

        io::stdin()
            .read_line(&mut max_number_input)
            .expect("Failed to read line");

        let max_number_input: u32 = match max_number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input...retry...");
                continue;
            }
        };

        if max_number_input == 0 {
            println!("Number should be higher than 0...Try again...");
            continue;
        } else {
            max_number = max_number_input;
            println!("Guess the number between 0 and {max_number_input}");
            break;
        }
    }

    let secret_number: u32 = rand::thread_rng().gen_range(1..=max_number);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input...😢...try again...");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small...🫣");
                println!("-----------------------");
            }
            Ordering::Greater => {
                println!("Too big...🍆");
                println!("-----------------------");
            }
            Ordering::Equal => {
                println!("YOU WIN 🚀! WHAT A HERO!!");
                break;
            }
        }
    }
}

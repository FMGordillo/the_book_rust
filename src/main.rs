use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn start_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You've won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}

fn main() {
    loop {
        start_game();
        println!("Do you want to play again? [Y/n]");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        match answer.trim().to_lowercase().as_str() {
            "y" | "yes" => start_game(),
            "n" | "no" => break,
            _ => continue,
        }
    }
}

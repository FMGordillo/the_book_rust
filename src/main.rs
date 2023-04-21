use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Log {
    guessed_number: u32,
}

fn start_game(logs: &mut Vec<Log>) {
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
        let log = Log {
            guessed_number: guess,
        };
        logs.push(log);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You've won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }

    println!("Do you want to see the logs? [y/N]");

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            println!("You've guessed for {} times", logs.len());
            for (i, log) in logs.iter().enumerate() {
                if logs.len() == i + 1 {
                    println!("{}", log.guessed_number);
                } else {
                    print!("{} | ", log.guessed_number);
                }
            }
        }
        _ => (),
    }
}

fn main() {
    loop {
        let mut logs: Vec<Log> = Vec::new();
        start_game(&mut logs);
        println!("Do you want to play again? [Y/n]");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        match answer.trim().to_lowercase().as_str() {
            "n" | "no" => break,
            _ => continue,
        }
    }
}

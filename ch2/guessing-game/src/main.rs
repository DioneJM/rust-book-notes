use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Guess a number!");
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parse error: {} ", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too Low"),
            Ordering::Equal => {
                println!("Noice.");
                break;
            }
            Ordering::Greater => println!("Too High"),
        }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess, or q to quit.");

        let mut guess = String::new();

        io::stdin().
            read_line(&mut guess).
            expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok (num) => num,
            Err(_) => {
                if guess.trim().eq("q") {
                    println!("Quitting.");
                    break;
                }
                println!("Invalid input, try again.");
                continue;
            }
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            },
            Ordering::Greater => println!("You guessed too high!")
        }
    }
}

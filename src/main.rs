use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Welcome to GGuess");
    
    let mut player_life = 3;

    let min_range = 1;
    let max_range = 10;
    println!("Guess the number between {} and {}", min_range, max_range);

    let secret_number = rand::thread_rng()
                                .gen_range(min_range..max_range);
    

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                player_life = player_life - 1;
                println!("{}", "Too small!".red());
                println!("You have {} live's left", player_life);
                if player_life == 0 {
                    println!("You loose.. Bye Bye");
                    break;
                }
            },
            Ordering::Greater => {
                player_life = player_life - 1;
                println!("{}", "Too big".red());
                println!("You have {} live's left", player_life);
                if player_life == 0 {
                    println!("You loose.. Bye Bye");
                    break;
                }
            },
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }

}

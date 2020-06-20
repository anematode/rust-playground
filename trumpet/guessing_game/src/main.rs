use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn format_count(count: i32, singular: &str, plural: &str) -> String {
    if count == 1 {
        return format!("{} {}", count, singular);
    } else {
        return format!("{} {}", count, plural);
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses: i32 = 10;

    loop {
        println!("Please input your guess.");
        println!("You have {} remaining.", format_count(guesses, "guess", "guesses"));

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
    	.expect("An error occurred!");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        match guess.cmp(&secret_number) {
        	Ordering::Less => print!("Too small! "),
        	Ordering::Greater => print!("Too big! "),
        	Ordering::Equal => {
                println!("You win! You may or may not get a free sheep.");
                break;
            }
        }

        guesses -= 1;

        if guesses <= 0 {
            println!("Sorry, you lost!");
            println!("In case you were wondering, the number was {}.", secret_number)
            
            break;
        } else {
            println!("Try again!");
        }
    }
}

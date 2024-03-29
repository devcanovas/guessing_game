use rand::Rng;
use std::cmp::Ordering;
use std::io;

const TOO_SMALL_MESSAGE: &str = "Too small!";
const TOO_BIG_MESSAGE: &str = "Too big!";
const YOU_WIN_MESSAGE: &str = "You win!";
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

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

        println!("You guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{TOO_SMALL_MESSAGE}"),
            Ordering::Greater => println!("{TOO_BIG_MESSAGE}"),
            Ordering::Equal => {
                println!("{YOU_WIN_MESSAGE}");
                break;
            },
        }
    }
}

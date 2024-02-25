use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);

    println!("I'm thinking of a number...");
    println!("Can you guess it?\n");

    loop {
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!\n");
                continue;
            },
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{guess} is too small!\n"),
            Ordering::Greater => println!("{guess} is too large!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 10001);

    // println!("secret number {}", secret_number);


    loop {
        let mut guess = String::new();
        println!("insert number");

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        println!("Guess: {}", guess);

        let guess_as_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_as_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }

    }
}

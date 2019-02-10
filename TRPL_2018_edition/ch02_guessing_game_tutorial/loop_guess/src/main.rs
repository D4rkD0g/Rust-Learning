extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn play_again() -> bool {
    
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Fail");
        let choice = choice.trim().to_ascii_lowercase();

        if choice == "y" {
            break true
        } else if choice == "n" {
            break false
        } else {
            println!("Input Y/N or y/n");
            continue;
        }
    }
}
fn main() {

    loop{
        let secret = rand::thread_rng().gen_range(1, 101);
        println!("Let's Start, Input a num[1-100]: ");

        loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                        .expect("failed to read");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_)  => {
                    println!("Need a NUMBER");
                    continue;
                }
                
            };

            match guess.cmp(&secret) {
                Ordering::Equal => {
                    println!("Yes, you win");
                    break;
                },
                Ordering::Greater => {
                    println!("NO, TOO Big");
                },
                Ordering::Less => {
                    println!("No, TOO Small");
                }
            }

        }

        println!("Continue???[Y/N or y/n]");
        if !play_again() {
            break;
        }
    }
    println!("Bye~~");
}

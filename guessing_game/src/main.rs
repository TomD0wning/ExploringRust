extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1,101);
    
    println!("Random number is {}",random_number);

loop{
    println!("Input your number");

    let mut guess  = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to readline");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
        

    println!("You guessed: {}", guess);

        match guess.cmp(&random_number){
            Ordering::Less => println!("To low"),
            Ordering::Greater => println!("To high"),
            Ordering::Equal => {
                println!("Spot on");
                break;
                }
        };
    }
}

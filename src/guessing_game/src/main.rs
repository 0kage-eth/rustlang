use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let mut guess = String::new();
		
    println!("Starting Guessing Game");
    println!("Please input your guess:");
	
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read value");

    println!("you guessed: {guess}");

    let guess : u32 = guess.trim().parse().expect("Invalid number guessed");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {secret_number}");
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("guess is too small"),
	Ordering::Greater => println!("guess is too big"),
	Ordering::Equal => println!("exact match. You win!!!"),
    }
}
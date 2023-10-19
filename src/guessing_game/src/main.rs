use std::io;

fn main() {
    
    let mut guess = String::new();
	
    println!("Starting Guessing Game");
    println!("Please input your guess:");
	
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read value");

    println!("you guessed: {guess}");
}
fn main() {
    // immutable and mutable variable declaration 
    let x = 5; // declares a immutable variable x
    let mut y = 10; // declares a mutable variable y

    println!("value of x is {x}");

    println!("value of y is {y}");
    y= 6;
    println!("value of y is {y}");// mutable variable gets reassigned;
    
    // constants

    const MAX_PLAYERS:u32 = 56;
    const SECONDS_IN_DAY:u32 = 60*60*24; // can define expressions also as constants
    // offer better readability
    println!("MAX PLAYERS = {MAX_PLAYERS}");
    println!("Seconds in day constant = {SECONDS_IN_DAY}");

    // shadowing

    let shadow_var: u32 = 10;
    println!("shadow var initial = {shadow_var}");

    let shadow_var : u32 = 4;
    println!("reassigning shadow_var = {shadow_var}");	 

}




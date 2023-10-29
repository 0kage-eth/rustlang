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
    {
       let shadow_var:u32 = 20;
       println!("reassigning shadow_var = {shadow_var}");	 
    }
    println!("reassigning shadow_var = {shadow_var}");
    
    // shadowing also can change daya type
    let shadow_var:i128 = 7666;
    println!("shadow var now {shadow_var}");
    
    let var_floating: f64 = 2.455;
    let var_def_floating = 3.444; // by fefault it is 64 bit
    let var_float_32bit: f32 = 2.333; // need to explicitly assign f32
    println!("float 32 bit val: {var_float_32bit}");


   // arithmetic operations
   let add2 = 32 + 64;
   let sub2 = 77 - 33;

   let truncated = 32/3; // division leads to 10
   let reminder = 55 %4; // reminder is 3
   
   let status: bool = false;
   
   // character type is most primitive type
   let var_char: char = 'A';

   let var_tup: (i32, f64, char) = (32, 2.344, 'g');
   let (t1, t2, t3) = var_tup;
   println!("value of floating point in tup {t2}");
   let x : (i32, f64, char) = (44, 3.555, 'k');
   let x_char = x.2;
   println!("name in the tuple {x_char}"); 
         
}




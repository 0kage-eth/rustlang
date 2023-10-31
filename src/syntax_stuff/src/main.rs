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
   let x_array: [u32; 5] = [1, 2, 3, 4, 5]; // defined an array with u32 with 5 elements
   let x_array_initialized = [3; 5]; // initializes x_array with size 5 and all elements 3
  let fifth_element = x_array_initialized[4];
  println!("fifth element {fifth_element}");         
  sub_func(); // calling function defined below
  let sub_val = sub_func2_return_val();
  println!("sub func return val: {sub_val}");
  let isGreater = sub_func_if(99);
  println!("is greater: {isGreater}");
  let multi_val: u32 = sub_else_if(99);
  println!("multi val from else if; {multi_val}");
  let loop_var:u32 = loop1();
  println!("final value of loop_var: {loop_var}");  
  let remaining:u32 = loop_with_labels();
  while_for();
}



    fn sub_func(){
     println!("inside sub func");
    }
    

    fn sub_func2_return_val() -> u32{
     5
    }// shortform to define a function
    // note that in this case, return value is defined as an expression
    // perfectly valid to assign it to a variable


    fn sub_func3_input_return_val(x: u32) -> u32{
     x+ 5
    }//note that this is an expression
    // if we place a semi colon after x+5, it becomes a statement
    // in that case, assigning it to a variable using let throws an error
    // very important classification between expression and statement 
    
    fn sub_func_if(val: u32) -> bool{
     if val > 50 {
       true
     }
     else{
      false
     }
    }
    fn sub_else_if(val:u32)->u32{
      if val > 1000{
       45
      }
      else if val>500{
       35
      }
      else if val > 100 {
       25
      }
      else{
       10
      }
    }


   fn loop1()->u32{
    let mut i: u32 = 0;
    loop{
     if i>32{ break;}
     i = i + 1;
     println!("current i {i}");
    }
    i+20
   }

   fn loop_with_labels() ->u32{
   
    let mut index:u32 = 0;
    'outer: loop{
       let mut remaining:u32 = 100;
       loop{
        println!("remaining: {remaining}");
        if remaining < 60{ break;}
        
        if index > 2 {break 'outer;}
        remaining -= 10;
       } 	       

       index += 1;
       println!("index val: {index}");
    }
    index
  }

  fn while_for(){
   let a:[i32;5] = [1,2,3,4,5];
   let mut i = 0;
   while i < 5{
     println!("current val of array {}", a[i]);
     i += 1;
   }
   for element in a{
     println!("currene element {element}");
   }

   for number in (1..5).rev() {
    println!("number in reverse {number}");
   }
  }

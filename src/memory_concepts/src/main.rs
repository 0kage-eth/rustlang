fn main() {


    //test_string_deep_copy();

    let s1 = String::from("hello");
    let s2 = test_pass_variable_to_function(s1);

    println!("value of s2 {}", s2);

    let (s3, u1) = pass_back_variable(s2);
    println!("length of string {u1}, value of string {}", s3);

    let mut s4 = String::from("Head ");
    let new_length = append_string(&mut s4);
    println!("new length {}, new string: {}", new_length, s4);
}

fn test_string_assignment(){
    let s1 = String::from("Hello There");

    let s2 = s1;
//    println!("Value of s1 {}", s1); // this will lead to an error
}

fn test_string_deep_copy(){
    let s1 = String::from("hello there");

    let s2 = s1.clone();

    println!("Value of s2 {}", s2);
    println!("Value of s1 {}", s1);// since we did a deep clone this should work
}

// function takes a string and returns it back
fn test_pass_variable_to_function(s1: String)-> String{
    let s2 = s1 + " world";
    s2 
}


fn pass_back_variable(s1: String)-> (String, usize){
    let len = s1.len();

    (s1, len)
     // note that even though we just needed length
     // we have to pass back s1 as s1 has lost its existence in caller context
     // this needlessly adds to function signature with no value as such
     // in this example it would have been better to use s1 as reference and not value	
}

fn calculate_length(s: &String) -> usize{
    let length = s.len();
    length
}

fn append_string(s:&mut String) -> usize{
  s.push_str(" tail");
  let length = s.len();
  length
} // note that we are no returning new_s because s is already updated


fn find_first_word(s:&String) -> usize{ // returns the size of string corresponding to first word
        
}

}

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

    let s5= String::from("Good heart wins good friendships");
    let s6 = slices(&s5, 10);
    println!("slices string {}", s6);     
    
    let (i, sliced_str) = find_first_word_with_index(&s5);

    println!("index of first space {i}, sliced string {}", sliced_str); 

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
    let s_bytes = s.as_bytes(); // converts string to an array of bytes

    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' '{ // denotes byte literal for space
            return i;
        }
    }
    s.len()
}


fn slices(s:&String, length:usize) -> &str{
 
 let s2 = &s[..length];
 &s2
}

// we can modify the find_first_word function to return both index and slice as follows

fn find_first_word_with_index(s:&String)-> (usize, &str){
     let s_bytes = s.as_bytes();

     for (i, &item) in s_bytes.iter().enumerate(){
       if item == b' '{
        return (i, &s[..i]);
       }
     }
     (s.len(), &s[..])

}

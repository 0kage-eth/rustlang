fn main() {
   
    let mut s = String::new();
    println!("New string {}", &s);
    

    let d = "some content"; // this defines a string literal
    println!("d: {}", &d);

    s = d.to_string(); // converts a string to String
    println!("s after assigning: {}", &s);

     
     let append_str = "...some more content";
     s.push_str(append_str);

     println!("new appended string: {}", &append_str);
     println!("s after appending: {}", &s);

     let mut m = String::from("0");

     m.push('K');
     m.push('a');
     m.push('g');
     m.push('e');
     
     println!("my name: {}", m);

     let mut addedS = s + &m; // note s is owned and can no longer be accessed
     println!("added s {}", addedS);


}



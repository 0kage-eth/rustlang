
use std::collections::HashMap;



fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 22);
    
    let team_name = String::from("blue");	
    let blue_score = scores.get(&team_name).copied().unwrap_or(0);	
    

    // note above that scores.get() gives an Option<&V>
    // this can be a some(V) or none(V)
    // copied() converts Option<&V> to Option<V>
   // unwrap_or converts this into V or 0

    println!("blue score {}", blue_score);


    for (key, value) in &scores{
       println!("{key} : {value}");
   }

   // updating an existing hashmap

   scores.entry(String::from("yellow")).or_insert(80);
   scores.entry(String::from("red")).or_insert(199);


  //in the above only yellow will be updated, red would be as is

    println!("{:?}", scores); // macro works for hash maps


   let s = "This is great really great";
   count_words(&s);
}


fn count_words(s: &str) {
   let mut map = HashMap::new();

   let s = "This is great really great";
   for word in s.split_whitespace(){
	let count = map.entry(word).or_insert(0);
        
        *count += 1; 

   }
   println!("{:?}", map);
}

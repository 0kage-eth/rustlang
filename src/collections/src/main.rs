fn main() {
   vector_playground();

}


fn vector_playground(){
   

  // defining a new vector
   let new_vec: Vec<u32> = Vec::new(); // Vec is part of standard library
   let new_vec_macro = vec![1,2,3,4]; //Rust automatically interprets the type in this case
   // vec! is a macro that allows easy creation of vectors
   

   // adding elements to an existing vector
   let mut new_vec2: Vec<u32> = Vec::new(); // make it mutable so we can add elements later

   new_vec2.push(33);
   new_vec2.push(44);
   new_vec2.push(55);
   //push is used to append values at the end of the new_vec vector

   // get the length of vector using .len()

    println!("length of new_vec2 {}", new_vec2.len());
   // reading elements in a vector

   let item: &u32 = &new_vec2[1]; // second element
   // note that we defined element by reference
   println!("second element in new vec2 {}", item);

   // using option to read via get()
   let optional_item: Option<&u32> = new_vec2.get(2);
   match optional_item{
     Some(u) => println!("third item {}", u),
     None => println!("third item does not exist"), 
   }

   // 2 reading methods described above differ in the way they respond for out of index elements
   // when reading out of index elements, &v[n] panics since it can't find the memory location
   // v.get(n) returns a None type -> cleaner way of exiting without panicing


   // looping over elements

   for i in &new_vec2{
       println!("value {i}");
   }

   // mutable loop

   for i in &mut new_vec2{
        *i += 10;
         println!("value {i}");
   }

 let variable_vec:Vec<CellType> = vec![CellType::INT(44), CellType::STR(String::from("gello")), CellType::FLOAT(2.233)];   
 for val in &variable_vec{
   println!("value is {:?}", val);

  }

}



// using vetcxors with enum

#[derive(Debug)]
enum CellType{
   INT(i32),
   STR(String),
   FLOAT(f64),
}





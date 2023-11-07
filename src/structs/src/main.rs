// Structs playground

fn main() {
    println!("Hello, world!");
    
    let mut s1 = Student{name: String::from("kage"), 
		  active: true, 
                  rollId: 76662, 
                  email: String::from("kage@kage.com")};


    println!("s1 name: {}", s1.name);
    println!("old roll id {}", s1.rollId);
    s1.rollId = 122222;
    println!("new roll id {}", s1.rollId);

   addRollId(&mut s1);
   println!("roll id after addRollId {}", s1.rollId);

   let mut s2 = createNewStudent(String::from("max"), 
				true, 
				566666, 
				String::from("max@madmax.com"));

   println!("New student email: {}", s2.email);

   let s3 = Student{name:String::from("barley"), ..s2};
   //println!("s2 email {}", s2.email); // this will not work because memory is moved in previous statement

   println!("s2 name {}", s2.name); // this will work however because we have not assigned name from previous instance
}


fn addRollId(student:&mut Student) -> u32{
  student.rollId += 100;
  student.rollId

}

fn createNewStudent(name: String, active: bool, rollId: u32, email:String ) -> Student{
  Student{name, active, rollId, email}
}

struct Student{
 name: String,
 active: bool,
 rollId: u32,
 email: String
}




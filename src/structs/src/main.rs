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

   println!("s2 name {}", s2.name); // this will work however because we have not assigned name from previous instanc

  let black = Color(0,0,0);
  let white = Color(255, 255, 255);

  let y_point = Point(10,0,0);
  println!("y_0: {}, y_1: {}, y_2: {}", y_point.0, y_point.1, y_point.2);
 
  // let mut p = Parent{name: "KageSr", rollId: 25};
  // println!("parent name {}", p.name);
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


struct Color(i32, i32, i32); // tuple struct instances
struct Point(i32, i32, i32);

struct Parent{
 name: String,
 rollId: u32, //rollId of student 
}

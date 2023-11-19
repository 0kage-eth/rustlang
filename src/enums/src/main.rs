fn main() {

    let tr = Shapes::TRIANGLE; // need to define it with two semi colons

    let v6 = IPAddress::V6;

    let localAddress = Address { iptype: IPAddress::V4, address: String::from("192.168.0.1")}; 

    let routerAddress = IPAddress2::V4(String::from("192.168.0.0"));
    

   let router4Generic = IPAddress3::V4(IP4Address{addressFormat: String::from("v4form"), id: 55});

   let router6Generic = IPAddress3::V6(IP6Address{numberFormat: String::from("v6num"), id: 232});


  // println!("router 4 generic address format {}", router4Generic.addressFormat);

  let msg = Message::Move{x:2, y:4};
  let d = msg.distance();

  println!("distance in message {}", d);

  let some_number = Some(5);

  let some_char = Some('f');

  let opt_number: Option<u32> = None;

  println!("optional number {:?}", opt_number);
}


enum Shapes{
    TRIANGLE,
    RECTANGLE,
    SQUARE,
    CIRCLE
}


enum IPAddress{
 V4,
 V6,
}


// define a struct on top of an enum
// note that this is using both structs and enums
struct Address{
    iptype:IPAddress, // note here type is of Enum type
    address:String,
}

// instead we can  make enum contain string
enum IPAddress2 {
    V4(String),
    V6(String),  
}


struct IP4Address{
    addressFormat: String,
    id: u32,

}

struct IP6Address{
   numberFormat: String,
   id: u32,
   
}


// generic enum that defines different structs for each enum type

enum IPAddress3{
    
    V4(IP4Address),
    V6(IP6Address),
}


enum Message{
  Quit,
  Move{x: u32, y: u32},
  Write(String),
  ChangeColor(i32,i32,i32),
}

// this enum has multiple different types 
// Quit has no associated data at all
// Move has name fields like a struct
// Write ncludes a single string
// ChangeColor includes 3 i32 values


impl Message{
   fn distance(&self)->u32{
     0
    // if(self == Message::Move) { self.x^2 + self.y^2} else { 0} 
   }
}









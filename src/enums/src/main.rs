fn main() {

    let tr = Shapes::TRIANGLE; // need to define it with two semi colons

    let v6 = IPAddress::V6;

    let localAddress = Address { type: IPAddress::V4, address: String::from("192.168.0.1")}; 

    let routerAddress = IPAddress2::V4(String::from("192.168.0.0"));

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
    type:IPAddress, // note here type is of Enum type
    address:String,
}

// instead we can  make enum contain string
enum IPAddress2 {
    V4(String),
    V6(String),  
}


## Enums

- Enums in rust can encode meaning along with data
- `Option` is a type of enum that is used to express that value can be something or nothing
- `match` helps to run different code for different values of enum

### Defining a enum


- Enums give you a way of saying a value is one of possible set of values
- Lets say Shapes Enum can take in circle, square, triangle, rectangle
- Enums are more appropriate than structs in such cases


- Another example is IP address, can be version 4 or version 6 address but not both at the same time

- enum once defined can be used inside a struct just as any other type. See example below


```
enum IPAddress{
    V4,
    V6.
}

struct Address{
    type: IPAddress,
    ip: String,
}


fn test(){
   let addy = Address{type: IPAddress::V4, ip: String::from("192.168.0.1")};
}

```

- Note that in the above struct is defined on top of an enum. However, Rust makes enums powerful by allowing them to have associated values. Foe eg. above can be done as follows


```
enum IPAddress{
   V4(String),
   V6(String),
}

// above allows for a more concise representation of above struct

fn test(){
   let router = IPAddress::V4(String::from("192.168.1.1"));
}



```

- Note above that a function is automatically created IPAddress::V4() and IPAddress::V6() that both take a string as an input to the function

- Another benefit of the above is that String format can be different for both types. And, we could have also defined a different type for each enum type
 

- Infact, V4 type has its own specs for defining an address and V6 type has its own spec to define its address, that the most generic way fo defining this is something like...



```
struct IPV4Address{

}

struct IPV6Address{

}

enum IPAddress{
    V4(IPV4Address),
    V6(IPV6Address),
}

```





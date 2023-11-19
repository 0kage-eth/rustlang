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



- Note that we can define completely different types inside a single enum


```
enum Message{
  Quit,
  Move{x: i32, y: 32},
  Write(String),
  ChangeColor(i32,i32,i32),
}

// this enum has multiple different types 
// Quit has no associated data at all
// Move has name fields like a struct
// Write ncludes a single string
// ChangeColor includes 3 i32 values


//***Note that this is same as defining 4 different structs as below

struct Quit //unit struct

struct Move {x: i32, y: i32}

Struct Write(String) // tuple struct

Struct ChangeColor(i32, i32, i32) // tuple struct

```

- We can also defiune methods on enum the same way we defined methods on struct, ie. using the `impl` block



### Option type

- Used when a value could be something or nothing
- Rust does not have the Null type that other programming languages have
- Problem with null value is that if you try to use a null value as a non null value, you end up with an error

- Rust does not have nulls but introduces the null concept via enums with Option type -> enum Option<T> is defined as follows

```
    enum Option<T>{
       None,
       Some(T)
    }
```


- Option<T> is in the prelude -> we don't need to include it in current scope
- its variants are also included, we can use Some or None directly without adding Option::None or Option::Some prefix
- <T> syntax represents a generic variant that can hold any type. For eg. Some<T> can apply on Some(String) or Some(u32) equally

- Note that compiler treats T and Option<T> as two different types. Casting one to the other wont work because compiler would not let us use a an Option<T> value as a definite value. 


```
 fn main(){
     let x: u32 = 20;
     let y:Option<u32> = Some(12);
     let z = x + y; // **** this leads to an error *****	
}

```

- When we have a i32, we can confidently assume that it has a value and can never be a null. But when we have Option<i32> we have to worry about possibly not having any value -> this kind of gives more control to the developer on how they want to treat a given variable... interesting design...



- In general, we always need to convert Option<T> to T before running any operations on T. 

- Getting T value from Some<T> -> this is where the methods defined on Option<T> come into play. We have code to handle different variants -> code that runs only if there is Some(T) value and code that runs only when value is null. In the initial case, the code can use the inner T normally


### Match control flow constuct

- `Match` allows to compare value against series of patterns and execute code based on a specific pattern match

- patterns can be made of literal values, variable names, wildcards -> power of match comes from expressiveness of patterns

- Values goes through each pattern in match and code block gets executed at the first instance of a pattern match

- Let's look at this example:


```

#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn CoinVal(c: &Coin)->u32 {
  match c {
     Coin::Penny => 1,
     Coin::Nickel => 5,
     Coin::Dime => 10,
     Coin::Quarter => 25,
  }

```

- In the above, we have 4 arms for match -> each arm has a pattern and some code, separatyed by `=>`
- if pattern does not match for a given arm, code goes to the next arm
- Code for each arm is an expression and the resultant value of expression in the matching arm is the value returned by match
- if code of arm runs into multiple lines, we use a curly bracket
- match should cover all possibilities if run over an enum -> otherwise compiler throws an error


### Match with Option<T>

- We can use match on Option<T> -> to assign different code for None and Some<T>


### Catch all


- catch all comes into play to handle exhaustive condition of `match` when the value is a primitive type that can take unlimited values

- for specific values, we can build arms and for all other values, we can capture within a single arm

```
fn get_val(x: u32) -> u32{
    
    match x {
	7 => 23,
        5 => 2,
        other => other + 32,
    }
}

```

- In the above case, we used `other` to package all other cases into 1. This allows us to use `match` because we have made it exhaustive


- catch all arm has to be placed at the end for the code to compile

- We can use `_` to represent catch all

- We can use empty tuple () to represent nothing



### if let

We can use the `if let` that combines both if and let to handle values of one match pattern and ignore rest


```
fn main(){
    let config_max = Some(80);;
    if let Some(max) = config_max{
      println!("some max is {}", max);
    }

}

// above function using if let exactly matches the following using match


fn main(){
    let config_max = Some(800);

    match config_max {
      Some(max) => println!("some max is {}", max),
      _ => (),

   }
}



```




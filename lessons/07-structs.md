## Structs


- Custom data type that allows packaging of data of different types
- Structs allow programmers to create meaningful grouping of different data types that makes programming more intuitive

- To define a struct, we use the same keyword as below

```
struct Student{
  name: String,
  id: usize,
  active: bool,
}
```

- We can define a mutable struct as follows:

```

   let mut s1 = Student{name: "kage", id: 121212, active: true);

   s1.id = 432433;
```


- Note that we cannot selectively mut some data in a struct. Either struct instance is completely mutable or it isn't

- We can return struct instance from functions as shown

```
fn createNewStudent(name: String, active: bool, rollId: u32, email:String ) -> Student{
  Student{name, active, rollId, email}
}
```

- When initializing a new instance, if the inputs of the function are the same, note that we didn't have to redefine the struct properties again -> we only used the variable names once because they matched with property names



- We can initialize an instance by using data of another struct instance, as follows



```

 fn main(){

   let s1 = Student{name: "s1", email: "s1", rollID: 12222, active: true};


   let s2 = Student{name: "s2", ..s1};
   // Note that this is copying all the other properties from s1 instance
   // however, important to note is the = here refers to moving
   // since s1 has email that is String type and that gets moves
   // if we now try to access s1, it is not possible, since s1 memory is cleared
     
  }

```

- the property that is moved to s2 via assignment (`=`) will be lost in s1. Memory corresponding to that property is cleared by memory allocator and no longer be accessible. Extrenely important that we avoid errors related to moving data in structs -> can cause complex issues


- Tuple struct instances can be defined without named variables defining struct properties.They are called Tuple structs because they look like tuples.



```
struct Color(i32, i32, i32);
```


- Unit struct - we also have a unit struct that have no properties attached to it (not sure why this would be needed). Docs say that one example for this is to implement behavior of this type such that every instance of this type is always equal to ecvery instance of other type (need to look into example). Such structs do not need any curly brackets while definition and can be assigned without any paranthesis

```
struct UnitStructExample;

fn main(){
  let c = UnitStructExample; // no paranthesis or curlies here
}
```



###  Ownership of data in structs


- A good design for structs is to own all the data if its properties. So instead of using &str (a string slice type by reference), we prefer a String which is immutable reference to the data. This would mean that any assigning of struct data would result in moving of data and the property will no longer have access to that data location 

- A good design is where data inside properties is valid as long as the struct is valid


- So following will give an error - compiler will complain that it does not have lifetime specifiers inside struct


```

struct Student{
 email: &str;
 rollnumber: u32;
}


fn main(){
    let mut s = Student{
		    email:"kaga@kage.com",
		    rollnumber:22,
		};

}

```

### Adding functionality with derived traits

- in the past lessons we extensively used println! macro -> using a println! on a struct will not work though because curly brackets tell println to use `Display`formatting that is not defined on struct

- this is not an issue with primitive types because `Display` is defined by default

- to correct this we use following statement

```
println!("printing rectange {:?}" rect);

```

- note that we changed `{}`to `{:?}`
- this still does not work -> error we see is that #[derive(Debug)] needs to be added to struct 
- Rust has functionality to print debug info but we have to explicitly opt in for that


- Note that just like `println!`, we hav`dbg!` - difference here is dbg takes ownership of variable where as println! takes the reference of variable. 


- `dbg!` is a great way to debug and print out what is happening at each step


---

### Associating functions with specific structs


- in the above example, we have seen area function -> but that function is not closely coupled with the Rectangle struct

- in order to do so, we define what are called `methods` -> methods, unlike functions, are defined within the scope of the struct.  first parameter of method is always `self`

- To define the scope of the struct, we use the `impl` keyword - everything inside `impl` is within the scope of Rectange

- `&self` here is short of `self:&Self` - within `impl`, Self is an alias for the type that the impl block is for -> **all methods must hve a first parameter self of Self type. Rust lets us abbreviate with just self**

- IF we want to change the instance that we've called method for we would pass `mut &self`instead indicating mutability of its instance

- using `self`allows to move ownership -> this is rarely used but possible. You would use this in rare situations where the method transforms the instance into something else and caller will not have access to the original instance after calling the method (not sure when this is needed)


- note that a method can share name with a property -> any invoking of a method needs the `()`. often when we do this, we use the method to get the value stored in property -> such functions are called getters (we see this concept in solidity as well)

- Rust does not implement getters automatically for struct fields (unlike in solidity)

- In C/C++ -> we have this important difference, `.` is used to call methods on object and `->` is used to call methods on a pointer to the object. In Rust, compiler automatically detects the way `self` was used in signature and basically either borrows, borrows with mutability or owns. Rust automatically adds `&`, `&mut` or `*` to the object so that object matches signature of method. 

So, if the signature has an object passed by reference, then following are same, although first one looks much cleaner


```
p1.get(&p2)
(&p1).get(&p2)

```

- Reading is `&p`, Mutating is `&mut p` and Consuming is `p`

- Associated functions that don't need `self` instance of struct can also be defined inside the `impl` block. 

- In this case, we can call the associated functions ddefined on a struct by using `::`

- For eg. we can define a function that returns a square object given its side


```
impl Rectangle{
 
   fn square(side: u32) -> Self{

	Self{length: side, breadth: side}
   }

}

```


And this can be used inside a main as follows

```
fn main(){
    let squarish_rectangle = Rectangle::square(10);
}

```  


- Note that we called the function even without any active Rectangle instance

- Note also that we can have multiple implementatioln blocks

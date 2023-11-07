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







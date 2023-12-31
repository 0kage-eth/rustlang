## Collections

- Rust standard library includes very useful data structures called collections

- collections can contain multiple values
- they are stored in heap and can be dynamic
- Each one has its own merits and costs - picking the right one for ocassion comes with experience
- 3 types 
	- Vector -> allows to store a variable number of values next to each other
	- String -> collection of characters
	- hash map -> associate a value to a particular key -> a particular implementation of more generic `map`



### Vectors

- Vector is represented as `Vec<T>`
- allow you to store more than one value of a single data type -> stored next to each other
- stored in memory (heap)
- used when we have variable number of items, eg, prices of items at a store, or list of participants in a conference etc.

- to create new vector
```
  let new_vec:Vec<u32> = Vec::new();

```

- Vec is implemented with generic type, ie. it can apply on any data type
- Rust does not know what kind of type we intend to store in Vector
- When we want to create a vector of specific type, we define T in Vec<T> as we have done above with u32
- Rust provides vec! macro -> when we define the elements, Rust automatically infers the datatype

```
   let new_vec2 = vec![1,2,3,4]; // Rust infers that vector is of <132> type 

```

- To add elements to a created vector, we can use the `push` method

```
   let mut new_vec = vec![1.2.3.4];

   new_vec.push(5);
   new_vec.push(6);
```

- if we want to change vector, we have to make it mutable.

- We can read a particular element as a reference by either using &vec[n] or vec.get(n)
- &vec[n] gets the n+1 element -> since it is indexed from 0
- vec.get(n) gets the optional type, can be some or none
- Two approaches above differ in how they handle reading non-existent elements
- get() returns None while &v[n] panics since it can't find the index
- Note that we cannot read an element in vector and also apend to it in the same scope -> this goes back to first principles where you cannot have mutable and immutable reference to same variable inside the same scope

- this is because of way vectors work -> vectors put all the elements next to each other in memory -> addingan element to the end would mean allocating memory and copying items to new space, if there is not enough room at current location -> in such case, reference to any element would be pointing to a deallocated space which is not allowed in rust

- We can use the `for` to loop over all elements in a vector

```
for i in &vec{

 println!("{i}");
}
```
Above gets each element from immutable reference and prints them



- We can also use for loop with mutable reference where we make changes to the array


```
for i in &mut v{

*i += 100;
}

```

- To change value that mutable reference refers to, we have to use the * dereferencer to get the value of i before we use the += operator


### Using Enum to store multiple types in a vector

- As seen above, typically a vector only stores a single data type
- To get around this, if we know all types that can be used in a vector, we can simply use the enum type




## Strings

- String is a collection of bytes
- Rust has only one `string` type in the core language -> and this a string slice `str`
- This is usually used in a borrowed state in the form of `&str`

- `String` type is provided by Rust's standard library rather than coded into the core language
- `String` is growable, mutable, owned, UTF-8 encoded string type
- When we refer to string, we are either referring to String or to string slice `str`
- string slices are references to UTF-8 encoded string data stored elsewhere



### String operations


- For creating a new String, we use

```
 let mut s = String::new(); //-n creates a mutable string
```


- To create a new `string` literal, we can simply assign a string as below

```
 let d = "some random content"; // this creates a string literal
```

We can use the `to_string` method which is available on any type that implements `Display` trait


- We can also use the `from` method on String, as we have been doing. to create a String from string literal as follows

```
    let s = String::from("initial random content");
```
 

- We can use the `push_str` method on String to append a string slice -> `push_str` takes a string slice because we don't necessarily want to take ownership of the input parameter. 

```
let mut s = String::from("0");
let s2 = String::from("kage");
s.push_str(s2); // s now has 0kage

// since push_str takes a string slice, s2 still exists in scope
// we can check this by a println! 

println("s2 exists? {}", &s2);

```
In the above, if push_str would have taken onwership of s2, we wouldn't have been able to run a print on s2 later


- We also have a `push` that takes a single character and adds to the string

```

 let mut s = String::from("0");

 s.push("K");
 s.push("a");
 s.push("g");
 s.push("e");

```

- `+` operator also works to add two strings -> you can either own/borrow variables while using `+` -> so be careful if you are owing variables as the original variable memory is gone

- when using `+` operator, left string has to be owned and right string passed as reference. This is because `+` operator uses an `add` function with following signature

```
fn add(self, s: &str) -> String {
``` 

- Note that the second argument is of `&str` but even if we pass `&String` Rust compiles without an error > the reason this is possible is because Rust coerces the `&String` to `&str`

- Note also that the first argument in `+` is owned rather than borrowed -> so it will no longer be valid after the + operation

- To add multiple strings `+` operator becomes confusing -> for such cases, `format` is the way to go 


```
fn main(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("value of added string {}", s);

}

```


```
fn main(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("value of s is {s}");
}
```
- Note that format! uses references - its much easier to use without the risk of transferring owenership


**How does Rust store Strings**

- It is noteworthy that Rust does not support accessing individual characters in a string. For eg, the following throws and error


```
fn main(){
    
    let s = String::from("hello");
    let s_char = s[2]; // trying to access a specific character leads to an error

}

```

- unicode scalare values may be made up of more than 1 byte
- it is important to note that for some UTf-8 encodings, indexing on strings will not work as each character has more than 1 byte







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

- Vector is represented as `Vect<T>`
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

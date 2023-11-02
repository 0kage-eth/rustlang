## Ownership

- Set of rules that govern how Rust program manages memory
- programs need to manage how they use computer memory -> some have garbage collectors that track no longer used memory, others expect programmer to allocate and free memory
- Rust uses ownership approach -> memoery is managed with a system of ownership with a set of rules the compiler checks
- ownership featrues will not slow down a program while its running

---

### Stack and Heap
_Stack_
- Stack and heap are parts of memory available for your program to use during runtime
- Both handle memory storage differently
- Stack stores values in the order it gets them and removes in opposite order(last in, first out LIFO)
- Best example is a stack of plates ordered by a waiter, one on top of other -> the last received plate is placed on top of the stack. And if the waiter has to remove a plate, he will remove the last one again from the top
- Adding data is called `pushing` to the stack -> removing data is called `popping` off the stack
- **All data stored on stack must have fixed and known size - unknown size at compile time or size that can change does NOT go into stack but the heap**

_Heap_
- Heap is less organized -> when you put data onto heap, you request a certain amount of space
- Memory allocator finds an empty slot in the memory that is large enough, marks it as being in use, and returns a pointer to that slot, which is the address of the location where data is stored
- this process is called "allocating" on the heap
- beacuse pointer to heap is known and fixed size, this pointer can be stored on stack
- Example here is a restaurant table -> when you reach a hotel, host finds you a spot based on the number of people expected. When some people turn up late, they can simply ask the host the table location to connect with the rest of the group
- In this analogy, if it was a heap, there was no need for a host as the group would have picked the empty seats and got themselves seated there (which never happens)


_Comparisons_
- Pushing to stack is faster than allocating to the heap
- In former, there is no need to search, while in latter, allocator needs to search for location to store new data
- for stack, location is always top of the stack
- in heap, allocator has to find big enough space and keep an accounting entry of which slot is occupied and its location

- Similarly, accessing data from heap is slower from that of stack
- If heap requires processing data across different locations, it becomes slowers because there is a lot of jumping around in memory
- Analogy here is of a server who keeps jumping tables to take orders -> so order 1 in table1 and order 2 in table 2 and back to order 3 in table 1 and so on - there is a lot of bookkeeping and back-and-forth here that slows a waiter down. Instead it is much easier to take all orders from table 1 and then move to table 2 -> that is what a stack kind of does
- much faster to handle memory on stack because it is all at one place
- When code calls a function -> including the pointers to the heap and function's local variables are pushed to the stack. When function execution is complete, these values get popped off the stack


### Challenges of memory management
- Programs face the following issues, specially related to memory on the heap
	- keeping track of what parts of code are using what data on the heap
	- avoiding storage clashes where 2 different parts of code access the same memory location
	- minimizing amount of duplicate data on heap
	- cleaning out no-longer used data so that we don't run up with out-of-memory issues

- Ownership in that sense is defining rules around memory management, specially the way things are organized, accessed and deleted in the heap
- Understanding ownership allows one to stop thinking actively about memory management because ownership rules are taking care of it

---

###Ownership rules

Here are some of ownership rules

- Each value in rust has an owner
- There can only be one owner at a time
- When owner goes out of scope, value is dropped

Scope here refers to the range within a program for which item is valid.

Consider the variable declared below:

```
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

```

Note that this variable has a limited scope, from the point it was defined to the time the `}` appears where the scope actively ends. 

### String Type

- Simple data types whicha are fixed size are managed on the stack -> they reside on the stack and can be easily copied into local variables when needed. Memory management here is not that challenging
- To really understand the way Rust manages heap, we need a more dynamic type that cannot be stored on stack. Hence we consider the string type where data is stored on the heap
- String size need not be known upfront and something the can change in runtime. To handle this we have `String` type. String can be obtained from string literal as follows

```
 let s = String::from("hello");
``` 
- Note that the variable is mutable and can be modified after declaration as follows:

```
  s.push_str("_world");
  print("{}", s);
```

- unlike string literals, note above that String type can be mutated, lets see why
- in string literal, the size is known at compile time, and it is immutable -> hence they are directly hardcoded into final executable. This makes accessing and working with them super fast
- this flexibility comes from the immutability of string literal

- we cannot insert a blob of memory into the binary knowing well that the value and size can change on runtime
- In order to support mutable, growable piece of text, we have to allot memory in heap that is unknown at compile time and  can change in runtime
- This poses 2 challenges
	- we need to request memory from allocator at runtime
	- once used, we need to return the memory back to allocator for reuse
- first part above is done by programmer when we call `String::from()` as shown above - this part is always handled by the programmer
- if language supports garbage collector (GC), GC will do the job of identifying which variables are no longer used and free up memory
- with no GC, we will have to actively manage memory and free it up as the need be
- Rust takes a different approach  -> memory is automatically returned once the variable that owns this memory goes out of scope
- When the variable goes out of scope, Rust calls a special function `drop` to return memory - it gets automatically triggered when `}` is processed
- 

  













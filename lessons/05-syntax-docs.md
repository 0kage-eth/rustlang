## Rust Syntax

### Variables

- `Let` allows users to assign variabels
- by default variables are declared as immutable
- rust compiler throws an error when variable is re-assigned a new value
- as we have seen before, to declare a mutable variable we use `mut` keyword

```
   let mut x = 5;
   let mut x
   x= 6;
``` 

---

### Constants
- constants are similar to variables in the sense that they are immutable
- not allowed to use the `mut` keyword with constants
- rust convention is to use all caps for variables declared as constants
- constants come with `const` keyword
- constants are valid for entire time the product runs and are useful to that extent

---

###Shadowing
- In rust we can declare a variable as the same name as a previous variable
- in that case, it is said that the "first variable is shadowed by second"
- it means that second variable is what the compiler would see when we use the name of the variable
- Note that shadowing ends when the scope of the variable ends (ie. it is inside a {})
- You can overshadow a variable multiple times
- Note that this is different from `mut` keyword that makes a variable mutable
- In that case, the variable can be changed later -> in the case of shadowing,
although we changed the variable, it is still immutable for the rest of the scope
- another difference here is that in case of shadowing, we are effectively creating a new variable -> hence, we can change the type of the variable. Incase of mutable variable, type of variable cannot be changed

###Data types
- Each variable must be declared with its type
- Rust is a statically typed language -> all variable names should be known in advance
- 2 data types - scalar type and compount type
- scalar type represents a single value -> rust supports 4 scalar types
- integers, floating point numbers, booleans and characters

_Integer_
- signed and unsigned integers
- 8/16/32/64/128/256 bit signed and unsigned integers allowed
- signed variant with n bits ranges from -(2^(n-1)) to 2^(n-1)-1
- integer overflows are checked in debug mode but not in release mode -> in debug mode, program panics when there is an overflow and program exits with an error. 
- in release mode, the integer is wrapped -> so overflow again starts from the lowest, similar to solidity
- 

_Floating Point_
- two primitives here - f32 and f64
- default floating point is f64

_Boolean_
- bool type - true/false

_Char_
- primitive type
- char literals are specified in single quotes -> string literals in double quotes


_Compound Types_
- two types supported - tuple and array
- tuple is grouping multiple data types into 1
- tuples are fixed length -> once declared, they cannot grow or shrink
- we can destructure a tuple by defining variables for each element in the tuple and then accessing that element
- we can also use period (`.`) to get specific element pf the tuple


_Arrays_

- arrays are defined using square brackets


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


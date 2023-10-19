# Guessing Game

- First step is to create a new cargo projector 

```
$ cargo new guesisng_game
```

- generates a Cargo.toml file as below

```
package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```


- Go to `src/main.rs` and start editing

- first add `std::io` library as follows

```
use std::io
```

- io library helps in reading a user given input

- any variable declared in rust is by default immutable, to make it mutable, we use the `mut` keyword

for eg.

```
let grapes = 5; // assigns value 5 to grapes variable -> and this is immutable
let mut grape = 5; // assignes value 5 to grape variable, but this time it is mutable

grape = 6; // this is possible -> since grape is declared as mutable variable
```


- in our case, we do

```
let mut guess = String::new();
```

we are assigning a mutable variable guess to a new empty instance of a String


- next we assign this guess variable to the guess input by the user. We achieve this by

```
io::stdin().read_line($mut guess)
```

this will use the function `stdin()` from the `io` module. And `read_line` helps read the user input and appends that into a string, without overwriting the contents. We pass the `guess` variable so that user input can be appended to the empty string - to do this operation, the string variable must be mutable and that's why we added the `mut` explicitly before guess. 

- Notice that we also used a `$` sign -> this is to tell that this argument is a reference pointer -> we can use the variable without needing to copy data to memory multiple times.

- Now the `read_line` returns a `Result` which is an `enumeration` - `Result` type can be in one of many possible states, called variants. In this case, the enum variants can be `Ok` or `Err`. `Ok` variant indicates operation is successful and inside `Ok` is the successfully generated value. `Err` means the operation failed, and inside `Err` contains information of how/why operation failed

- Instance of `Result` type have methods defined on them -> `expect` is one such method. If `Result` is Err value, then `expect` throws an error with a message contained in `Err`. Instead, if `Result` is Ok, `expect` returns the value that `Ok` is holding. 

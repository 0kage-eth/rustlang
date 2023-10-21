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

-  To display inline message with print we use

```
println!("your guess, {guess}");

```

- To display arithmetic calculated in print statement

```
println!("x={x}, y={y}, x+y={}", x + y);
```


- Check that the project compiles and we get the expected output at this stage

```
bash-3.2$ cargo build
   Compiling guessing_game v0.1.0 (/Users/apple/Documents/rustlang/projects/rust_lang/src/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.85s
bash-3.2$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Starting Guessing Game
Please input your guess:
44
you guessed: 44
```


- As next step, we need to implement a random number that acts as a system value on every run
- We don't want to implement a random number generator -> hence use a package -> in cargo
world, its called a crate
- Crate is a collection of rust code files
- project we are building so far is a binary crate -> to use external rst code files, we need a library create
- first step to use `rand` crate is to modify the cargo.toml file and add following to dependencies


```
[dependencies]
rand = "0.8.5"

```

- In the above, 0.8.5 represents a semantic version and means any patch released over 0.8.0* and 
until version changes to 0.9.0 - version above 0.9.0 is not guaranteed to have same API

On building the project, we note that 7 crates have been downloaded. These are dependencies needed to run rand 0.8.5

```
bash-3.2$ cargo build
    Updating crates.io index
  Downloaded rand_chacha v0.3.1
  Downloaded rand v0.8.5
  Downloaded libc v0.2.149
  Downloaded ppv-lite86 v0.2.17
  Downloaded getrandom v0.2.10
  Downloaded rand_core v0.6.4
  Downloaded cfg-if v1.0.0
  Downloaded 7 crates (905.8 KB) in 1.10s
   Compiling libc v0.2.149
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.10
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/Users/apple/Documents/rustlang/projects/rust_lang/src/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.26s
```


- Cargo fetches all dependencies from regsitry, which is a copy of Crates.io
- `Cargo.lock` file locks the file versions so that we get a reproducible build every time we compile. this file stores the version numbers for the first time and keeps using the same versions (even though an update with a breaking change is in place) 

- Unless we phusically update, `Cargo.lock` tries to use the same versions

- to update crates to a new version -> we use 
```
$ cargo update		
```

- Next, we first use the rand::Rng library in the current file

```
use rand::Rng;
```
This is along with the std::io library. Note that rand is library (or crate) and
Rng is a trait we are using along with this



- next we add a secret number that is generated using random numbers


```
let secret_number = rand::thread_rng().gen_range(1..=100);
```

Note that we are using rand::thread_rng() for generating a particular random number. Then we call the gen_range method on random number generator. This method is defined by Rng trait we brought into scope using rand::Rng statement at the top.


- A cool feature is 

```
$ cargo doc --open
```

gets all documentation related to dependencies we are using and opens in a browser. This helps us understand what is inside each library, traits and how to use them


- Next we add the "std::cmp::Ordering" that uses cmp library and Ordering trait
- `Ordering` has three variants -> Less, Greater and Equal
- Another important thing is if we have to compare secret_number with guess, we have to first convert
guess from string type to u32
- Rust allows shadowing -> you can use the same variable name to convert the variable into a different type

- Furst we do the following

```
let guess : u32 = guess.trim().parse().expect("Invalid number");

```


- note that we first trimmed to remove any spaces
- then we parsed to parse out a string to a number
- again this returns a `Result` trait that has `Ok` or `Err`
- We use expect to throw out a error message on error


- Note also, that we explicity cast guess to u32 (by default, casting is i32 but rust is intelligent enough to predict the type

lastly we make a check using `match`

```
  match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```


note above that cmp compares guess with reference pointer of secret_number (not copying to memory always). There are 3 outcomes -> for each pattern (ourcome), we can give a specific output message

On ruinning, we get everything working as follows

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```
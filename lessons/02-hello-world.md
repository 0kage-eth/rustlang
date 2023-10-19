## Hello World 

- To create first hello world application, here are the steps

	- create a new directory `$makedir hello_world` and go inside `$cd hello_world`
	- next create a `.rs` file called `hello.rs` - inside this, we write
	
	```
		fn main(){
		    println!("Hello World");	
		}	

	```
      - Now, compile `main.rs`with `$rustc main.rs`


- Points to note
	- `main` is a special function - it is always the first code that runs in every executable Rust program
	- println! here reperesents a macro, the ! here stands for macro. without !, this would have been a function
	- all lines end with a ;
	- on compiling, an executable is created that can be shared with anyone, who can run the program even without rust installed
	- Rust is ahead-of-time compiled language - when running a rust file, an executable gets created that can be shared with others who can run it even without rust installed. This is unlike python or js where sharing a .py or .js file would mean that the recipient needs to have the compiler installed
	
use std::fs::File;
use std::io::ErrorKind;


fn main() {
    
    let create_greeting = File::open("hello.txt");

    // return type here is of Result<T,E> type
    
    // first level match is on Result	
    let status = match create_greeting{
	
	Ok(file) => file, // returns file handle
	Err(error) => match error.kind(){
		ErrorKind::NotFound => match File::create("hello.txt"){
			Ok(fc) => fc, // creates file handle and returns
			Err(e) => panic!("could not create hello.txt"),
		},
		other_error => panic!("Problem opening file {:?}", error),
        }
    }; // error.kind() is returning a ErrorKind enum
// which has a type ErrorKind::NotFound -> in this case we are creating anew file

} 



// you can use closures instead -> unwrap_or_else

fn unwrap_example() {
   
    let create_greeting = File::open("hello.txt").unwrap_or_else(|error| {
	if(error.kind() == ErrorKind::NotFound){
		File::create("hello.txt").unwrap_or_else(|error| {
			panic!("Problem creating file {:?}", error);
		});
	}
	else{
		panic!("Problem opening file {:?}", error);
	}	
    });	
    
   
}

fn expect_and_unwrap() {

   let create_greeting_unwrap = File::open("hello.txt").unwrap();
   // panics if it cant find a file

  let create_greeeting_expect = File::open("hello.txt").expect("hello.txt not found");


}


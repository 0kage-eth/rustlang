fn main() {

    let coin = Coin::Dime;
    let coinValue = CoinVal(&coin);

    println!("Coin  is {:?} and value is {}", coin, coinValue);
    
    let coinV2 = CoinV2::Quarter(State::Florida);
    CoinValWithState(&coinV2);

    let s = Some(5);
    let n = None;
    let new_s = plus_one(s);
    let new_n = plus_one(n);
    
    catch_all_example(25);
    
}


#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn CoinVal(c: &Coin)->u32 {
  match c {
     Coin::Penny => 1,
     Coin::Nickel => 5,
     Coin::Dime => 10,
     Coin::Quarter => 25,
  }
}

#[derive(Debug)]
enum State{
   Alaska,
   Alabama,
   Pennsylvania,
   Florida, 
}


#[derive(Debug)]
enum CoinV2{
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}



fn CoinValWithState(c: &CoinV2) -> u32 {
    match c {
         CoinV2::Penny => 1,
         CoinV2::Nickel => 5,
         CoinV2::Dime => 10,
         CoinV2::Quarter(state) => {
            println!("State {:?}", state);
	    25 
         }
    }
}



fn plus_one(x: Option<u32>) -> Option<u32> {
    
   match x{
       None => None,
       Some(i) => Some(i+1),
   }


}



fn catch_all_example(x: u32) -> u32{

  match x{
     3 => run3(),
     5 => run5(),
     _ => runothers(),
  }

}


fn run3()->u32{println!("run3"); 3}
fn run5()-> u32{println!("run5"); 5}
fn runothers()-> u32{println!("run others"); 99}

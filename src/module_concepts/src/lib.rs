mod front_of_house;


fn deliver_order(){}

mod back_of_house{
    pub struct Breakfast{
	pub toast: String,
        fruit: String,
    }

   impl Breakfast{
       pub fn make(toast_choice: &str) -> Breakfast{
	    Breakfast{toast: String::from(toast_choice), fruit: String::from("peaches")}
       } //-n only toast can be chosen, not the fruits
	// say toast is chosen by customer and fruit is given by chef (use case)
	
      	
   }

   fn cook_order(){}
  
   fn prepare_order(){
      cook_order();
      super::deliver_order();
   }

   #[derive(Debug)]
   pub enum Appetizer{
	Soup,
	Salad,
	Starters,	
   }
    

}


pub fn eat_at_restaurant(){

     // absolute path
     //crate::front_of_house::hosting::add_to_waitlist();

     // relative path
     front_of_house::hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::make("brown");

     println!("current toast of breakfast {}", meal.toast);

     meal.toast = String::from("Wheat");

     println!("new toast of breakfast {}", meal.toast);

    //  println!("seasonal fruit {}", meal.fruit);

    let mut appetizer = back_of_house::Appetizer::Salad;

    println!("appetizer chosen {:?}", appetizer);


    appetizer = back_of_house::Appetizer::Soup;
    println!("appetizer changed to {:?}", appetizer);


}

pub trait Summary{
        fn author_info(&self) -> String;
	fn summarize(&self) -> String{
		format!("content: {}", "no content")
	} // this is like an interface in solidity
}


pub struct NewsArticle{
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
}

pub struct Tweet{
        pub username: String,
        pub content: String,
        pub retweet: bool,
        pub reply: bool,
}


impl Summary for NewsArticle{
   fn author_info(&self) -> String{String::from("")}
   fn summarize(&self) -> String{
        format!("{} by {} ({})", self.headline, self.author, self.location)
   }
}


impl Summary for Tweet{
	fn author_info(&self)-> String{
		format!("author@ {}", self.username)
	}
}

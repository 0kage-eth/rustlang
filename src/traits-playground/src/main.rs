use aggregator::{Summary, NewsArticle,Tweet};

fn main() {
	
	let mut news = NewsArticle{headline: String::from("Kage rockz"), location: String::from("crypton"), author: String::from("kage"), content: String::from("kage is happy")};

	let news_summary = news.summarize();
	println!("news summary: {}", news_summary);

	let mut tweet = Tweet{username: String::from("kage"), content: String::from("kage tweet"), retweet: false, reply: false};

	let tweet_summary = tweet.summarize();
	println!("tweet summary: {}", tweet_summary);

	let tweet_author = tweet.author_info();
	println!("tweet author: {}", tweet_author);

}






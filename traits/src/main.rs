use std::iter::Sum;

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("No author.")
    }

    fn default_implementation(&self) -> String {
        String::from("I'm some default implementation")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{} is the author", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.default_implementation());
    println!("1 new tweet: {}", tweet.summarize_author());

    let article = NewsArticle {
        headline: "Man learns Rust".to_string(),
        location: "London".to_string(),
        author: "Haydn".to_string(),
        content: "Probably the best article you will read.".to_string(),
    };

    notify(&tweet);
    notify(&article);
    notify_multiple(&tweet, &article);
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn return_summary() -> impl Summary {
    NewsArticle {
        headline: "Man learns Rust".to_string(),
        location: "London".to_string(),
        author: "Haydn".to_string(),
        content: "Probably the best article you will read.".to_string(),
    }
}

use std::fmt::Display;

use traits::{Summary, Tweet};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x is better x = {}", self.x);
        } else {
            println!("y is better y = {}", self.y);
        }
    }
}

fn main() {
    let one_tweet: Tweet = Tweet {
        username: String::from("@seolso"),
        content: String::from("How are you?"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", one_tweet.summarize());
}

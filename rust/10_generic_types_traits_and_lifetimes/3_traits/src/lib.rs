use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        return format!("More from {}", self.summarize_author()); // default behvaiour; can also
                                                                 // call a method from the same
                                                                 // impl
    }
}

// the below could be written like this, but
pub fn a<T: Summary>(_item: T) {}

// this is less verbose. Use this.
pub fn notify(item: &impl Summary) {
    println!("BREAKING: {}", item.summarize());
}

// this is a bit verbose.
pub fn asdf(_item1: &impl Summary, _item2: &impl Summary) {}

// in the case of above, use the generic types.
pub fn asdfasdf<T: Summary>(_item1: T, _item2: T) {}

// if you want to accept a variable that is implemented by two different traits..
pub fn asdfasfd(_item1: &(impl Summary + Display)) {}
pub fn asdfasfdasdf<T: Summary + Display>(_item1: &T) {}

// more clearer the second one is better
pub fn fwqe<T: Display + Clone, U: Clone + Debug>(_item1: T, _item2: U) {}

pub fn agww<T, U>(_item1: T, _item2: T) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// if you want to return an impl
fn get_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn asdlfkj() {
    // get_summarizable().username // impossible since the return type of `get_summarizable` is the
    // `imple Summary`, NOT Tweet.
}

// This code is impossible, not could not understand.
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

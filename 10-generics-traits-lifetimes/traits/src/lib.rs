use std::fmt::{Debug, Display};
use std::iter::Sum;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/// This will use the default summarize method
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub trait Summary {

    fn summarize_author(&self) -> String;

    /// this is default implementation
    /// in case if a type implements this trait
    /// but doesn't override this
    /// default implementation can call methods from the
    /// same trait
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

/// here item is some type that implements Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize())
}

/// Trait Bound Syntax
/// here any Type T implements Summary trait
/// similar to the previous notify function
/// but more verbose
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize())
}

// here params can be of different types
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// here params must be of same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

/// We can specify multiple trait bounds
/// with + syntax

pub fn notify_v3(item: &(impl Summary + Display)) {}

/// + is also valid with trait bound syntax

pub fn notify_v4<T: Summary + Display>(item: &T) {}

// this makes the function signature hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}

/// clearer trait bound with where clauses
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
    0
}

// returning Type that implements trait/s
// here the caller doesn't have to know the
// type being returned
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

// return multiple type that implements a trait/s
// wouldn't work and won't compile
fn return_summarizable(switch: bool) -> impl Summary{
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL"
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people"
            ),
            reply: false,
            retweet: false,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x:T, y: T) -> Self {
        Self { x, y }
    }
}

// trait bound to conditionally implement methods
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// conditionally implementing a trait
// for any type that implements another trait

impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{T}")
    }
    // body
}
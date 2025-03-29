use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // third lifetime elision rule applies here
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    // dangling reference
    // lifetime of r annotated with 'a and 'b for x
    // let r;          // ---------+-- 'a
    // //          |
    // {                     //          |
    //     let x = 5;   // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    // // program is rejected //          |
    // // since lifetime of b is shorter than a
    // println!("r: {r}");   // ---------+

    // fixes dangling reference
    let x = 5;            // ----------+-- 'b
                                //           |
    let r = &x;           // --+-- 'a  |
                                //   |       |
    println!("r: {r}");         //   |       |
                                // --+       |
                                // ----------+

    // Generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // Lifetime annotation syntax
    // &i32;        //  a reference
    // &'a i32;     // a reference with a lifetime
    // &'a mut i32; // a mutable reference with a lifetime

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}",result);
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // this won't work
    // println!("The longest string is: {}",result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt{
        part: first_sentence,  // here reference is valid as novel doesn't go out of scope until after the Important Excerpt instance
    };

    // this reference can live for the entire duration of the program
    // lifetime of all string literals is 'static
    let s: &'static str = "I have a static lifetime.";

}

// Generic Types, Traits and lifetimes together

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Lifetime annotation in function signature
// 'a will have the smaller of the lifetimes
//  of the references passed in to the function
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// if you're going to return a reference with a
// lifetime a i.e x then there is no need to
// specify lifetime for y as it doesn't have any
// relationship with lifetime of x or the return value
fn longest_v1<'a>(x: &'a str, y: &str) -> &'a str { x }

// fn longest_v2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     // return value must match the lifetime of at least one parameter
//     // as result will go out of scope at the end of function
//     // dangling reference
//     result.as_str
// }
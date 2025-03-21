mod front_of_house;
mod back_house;

fn deliver_order() {}

// a shorter path with use keyword
// re-exporting with pub use
pub use crate::front_of_house::hosting;

// idiomatic use Path
use crate::front_of_house::hosting::add_to_waitlist;
mod sample {
    use crate::front_of_house::hosting;

    pub fn does_use_work() {
        hosting::add_to_waitlist(); // nope it doesn't unless we use it in our module
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    // Relative Path
    back_house::cleaning_dishes();

    // using a shorter path
    hosting::add_to_waitlist();

    // idiomatic path use
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this won't compile as we aren't allowed to see or modify this
    // meal.seasonal_fruit = String::from("cranberries")

    let order1 = back_house::Appetizer::Soup;
    let order2 = back_house::Appetizer::Salad;
}
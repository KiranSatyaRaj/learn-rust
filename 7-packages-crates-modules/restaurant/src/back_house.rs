fn fix_incorrect_order () {
        cook_order();
        // Relative paths using super
        super::deliver_order();  // it's like using .. in linux
}

fn cook_order() {}
pub fn cleaning_dishes() {}

// Making structs and enums public
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}


// enums
// here both variants of Appetizer are public
pub enum Appetizer {
    Soup,
    Salad,
}

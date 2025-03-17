use crate::UsState::Alabama;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    // special expression for a few values
    // and default expression for all other values
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // a.k.a catch-all arm
    };

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),         // a placeholder _ if we don't want to use the value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),        // doesn't run any code in this case
    };

    // Matches with Ownership
    let opt = Some(String::from("Hello world"));

    match opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };

    match opt {
        Some(s) => println!("Some: {s}"), // opt has been moved to s here
        None => println!("None"),
    };

    // println!("{:?}", opt); // so this won't compile

    let opt = Some(String::from("Hello world"));
    match &opt {
        Some(s) => println!("Some: {}",s),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    // Concise control flow with if let
    let config_max = Some(3u8);
    match config_max  {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    };

    // a shorter way
    // syntax sugar for match
    // only runs for a single match pattern
    // ignores all other values
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
        let coin = Coin::Quarter(UsState::Alaska);
        match &coin {
            Coin::Quarter(state) => println!("State quarter from state {state:?}"),
            _ => count += 1
        }

        // using if let
        if let Coin::Quarter(state ) = &coin {
            println!("State quarter from state {state:?}");
        } else {
            count += 1;
        }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from state {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match must cover all possibilities
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
    // for example
    // this alone wouldn't work
    // which make match exhaustive
    // match x {
    //     Some(i) => Some(i + 1),
    // }
}
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory{
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // some other ways to annotate
    // all have same behaviour
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("Hello world"));

    // can't call with integer type since
    // the type inferred by compiler for
    // example_closure is String
    // let num = example_closure(4);

    // Capturing References or Moving Ownership

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // immutable borrow
    let only_borrow = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrow();
    println!("After calling closure: {list:?}");

    // mutable borrowing in closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut mutably_borrow = || list.push(4);
    mutably_borrow();

    println!("After defining the closure: {list:?}");

    // moving ownership to closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);

    println!("{list:#?}");

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     // sort_operations.push(value); // this implements FnOnce trait so it wont compile
    //     r.width
    // });

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // this uses FnMut
        r.width
    });

    println!("{list:#?}, sorted in {num_sort_operations} operations");

    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    // drop(s_own); // now s_own can't be dropped as long as clone in use
    cloner();

    // Processing a series of items with iterators

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let v1: Vec<i32> = vec![1, 2, 3];

    // this is an iterator adapter
    // map doesn't consume the iterator
    // rather it produces new iterator
    // each item in vector is incremented by 1
    // collect method consumes iterator and collects
    // the resultant value into a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

fn make_a_cloner(s_ref: & str) -> impl Fn() -> String + '_{
    move || {
        s_ref.to_string()
    }
}
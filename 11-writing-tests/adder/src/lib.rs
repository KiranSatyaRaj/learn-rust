pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String{
    // format!("Hello {name}")
    String::from("Hello!")
}

fn add_two(num: i32) -> i32 {
    println!("just printing");
    num + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0  {
            panic!("Guess value must be less than or equal to 100, got {value}");
        } else if value > 100 {
            panic!("Guess value must be greater than or equal 1, got {value}")
        }
        Guess{
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Julia");
        assert!(
            result.contains("Julia"),
            "Greeting did not contain name, value was `{result}`"
        )
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(3);
        // assert_ne!(result, 6); // asserts left != right
        assert_eq!(result, 5);    // asserts left == right
        // assert_eq!(6, result) // this works too
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
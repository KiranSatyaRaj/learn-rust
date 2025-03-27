
// generic Types in struct definitions
struct Point<T> {
    x: T,
    y: T,
}

// generic Types in method implementation

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // as this is already defined for specific type
    // rust compiler will reject this program
    // as rust doesn't know which implementation to use
    // fn distance_from_origin(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
}

// implement methods only on specific types
// instances with type Point<f32> will have this method
// and other instances will not
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// multiple generic Types in structs
struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

// generic Types in enums

enum Option<T> {
    Some(T),
    None,
}

// multiple generic Types in enums

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![10, 110, 414, 8411, 32];

    let result = largest_i32(&number_list);
    println!("The largest is {result}");

    let char_list = vec!['y', 'a', 'm', 'p'];
    let result = largest_char(&char_list);
    println!("The largest is {result}");

    println!(
        "The largest number is {} and The largest char is {}",
        largest(&number_list),
        largest(&char_list),
    );

    let integer = Point{x: 5, y : 5};
    let float = Point { x: 1.0, y: 2.0};

    // this won't work, unless the struct uses
    // multiple generic params
    // let wont_work = Point{x : 4, y: 0.2};

    let will_work = Point2{x: 24.2, y: String::from("Hello")};

    let p1 = Point3{x: 'c', y: 10.5};
    let p2 = Point3{x: "Hello", y: "char"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {} and p3.y = {}", p3.x, p3.y);


}

// This won't compile, the rust compiler
// expects us to define the capabilities
// the generic type T, without restrictions
// type T has no capabilities
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest  {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}
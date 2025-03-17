// add more meaning: Use structs

#[derive(Copy, Clone, Debug)] // debug trait that helps print the struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of rectangle is {} square pixels.",
        area(width, height)
    );

    let rect1 = (30, 50);
    println!(
        "The area of rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // using struct is much better than
    // trying to infer whether rect1.0 is width or is it height
    // now we have better idea of what a field refers to
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The rect is {:?}",
        rect1
    );
    // pretty-printing
    // easier-to-read while debugging
    println!(
        "The rect is {:#?}",
        rect1
    );
    println!(
        "The area of rectangle is {} square pixels.",
        area_using_struct(&rect1) // we want main to retain ownership of rect1
                                  // so we pass an immutable reference
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let a = area_using_deep_copy(rect1);
    println!("{} * {} = {}", rect1.height, rect1.width, a);
}

fn area_using_deep_copy(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn return_val(mut x: i32) -> i32 {
    x += 5;
    x
}

// much better way
fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// better way
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
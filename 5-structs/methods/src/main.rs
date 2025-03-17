#[derive(Copy, Clone, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Rectangle2 {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle2 {
    fn max(self, other: Rectangle2) -> Rectangle2 {
        Rectangle2 {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
            name: String::from("max"),
        }
    }

    fn set_to_max(&mut self, other: Rectangle2) {
        // let max = self.max(other);
        // *self = max;
    }
}

// All function within implementation i.e; impl block
// are Associated functions of struct Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold_another_rect(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

impl Rectangle {
    // This function doesn't need an instance of it's type
    // Such methods are used as constructors
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn max_rect(self, other: Rectangle) -> Rectangle{
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle)  {
        let max = self.max_rect(other);
        *self = max;
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rect1 is {}",
        rect1.area(),
    );

    println!("{:#?}", rect1);
    println!(
        "The width of rect1 has nonzero width: {}",
        rect1.width(),
    );

    let rect2 = Rectangle {
        width: 20,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 60,
    };
    println!("Can rect1 hold rect2: {}", rect1.can_hold_another_rect(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold_another_rect(&rect3));

    let sq = Rectangle::square(5);
    println!(
        "The area of sq is {}",
        Rectangle::area(&sq)
    );

    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

    let rect = Rectangle {
        width: 9,
        height: 9,
    };

    println!("{}", rect.area());
    let other_rect = Rectangle {width: 1, height: 1};
    let max_rect = rect.max_rect(other_rect);
    // we cannot use rect after it's moved
    // unless it implements Copy trait
    println!("{:?}", rect);

    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    // rect.set_width(3); // this won't work as rect itself is immutable

    let mut rect = Rectangle {
        width: 3,
        height: 4,
    };
    rect.set_width(4); // this works
    let rect_ref = &rect;
    // rect_ref.set_width(5); // this is not ok
}
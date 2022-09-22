struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("someoneelse@example.com"),
        username: String::from("someoneelse"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rectangle is {:#?}", rectangle);

    dbg!(&rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area(),
    );

    if rectangle.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle.width)
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rectangle1 hold rectangle2? {}", rectangle.can_hold(&rect2));

    let square = Rectangle::square(30);

    println!("square is {:#?}", square);
}

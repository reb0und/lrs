// Calculate area of a rectangle

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        length: 50,
        width: dbg!(30 * scale),
    };

    let rect2 = Rectangle {
        length: 50,
        width: dbg!(30 * scale),
    };

    println!("{}", rect1.can_hold(&rect2));

    println!("{rect1:#?}");

    println!("area of rect is {} sq px", rect1.area());

    if rect1.width() {
        println!("rect width is > 0");
    }

    dbg!(rect1);

    let sq = Rectangle::square(10);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}

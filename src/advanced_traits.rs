use std::fmt;
use std::ops::Add;

// trait Add<Rhs=Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Meters) -> Self::Output {
        Milimeters(self.0 + (other.0 * 1000))   
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("called pilot on human");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("called wizard on human");
    }
}

impl Human {
    fn fly(&self) {
        println!("called fly on human");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4))
    }
}

impl OutlinePrint for String {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("a baby dog is called a {}", <Dog as Animal>::baby_name());

    let hi = String::from("hi");
    hi.outline_print();

    let w = Wrapper(vec![String::from("hi"), String::from("hello")]);
    println!("{w}");
}

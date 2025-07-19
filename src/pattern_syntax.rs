struct Point {
    x: i32,
    y: i32,
}

struct NewPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Hello { id: i32 },
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    let x = Some(1);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("{y}"),
        _ => println!("default case, x = {x:?}"),
    }

    println!("x = {x:?}, y = {y}");

    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ascii letter"),
        'k'..='z' => println!("late ascii letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 1 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(1, y);

    match p {
        Point { x, y: 0 } => println!("on the x axis x = {x}"),
        Point { x: 0, y } => println!("on the y axis y = {y}"),
        Point { x, y } => {
            println!("on neither axis ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 100, 255));

    match msg {
        Message::Quit => {
            println!("quit");
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("{r}, {g}, {b}");
        },
        _ => println!("other"),
    }

    foo(1, 2);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("cannot overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (1, 2, 3);

    match numbers {
        (_, second, _) => println!("{second}"),
    }

    let _x = 10;
    let y = 10;

    let origin = NewPoint { x: 0, y: 1, z: 2 };

    match origin {
        NewPoint { x, .. } => println!("x is {x}"),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("matched n = {n}"),
        _ => println!("{x:?}"),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("found an id in range: {id_variable}"),
        Message::Hello {
            id: 8..=12
        } => {
            println!("id in another");
        }
        _ => println!("other"),
    }
}

fn foo(_: i32, y: i32) {
    println!("this code uses the y param {y}");
}

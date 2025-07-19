fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using favorite color {color}");
    } else if is_tuesday {
        println!("tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("purple");
        } else {
            println!("orange");
        }
    } else {
        println!("using blue");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at {index}");
    }
}

fn main() {
    let point = (1, 2);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({x}, {y})");
}

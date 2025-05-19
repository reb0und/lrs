// Loops
fn main() {
    let x = loop {
        break 1;
    };

    let mut count = 0;
    'ls: loop {
        count += 1;
        if count % 2 == 0 {
            break 'ls;
        }
    }

    let mut counter = 1;
    while counter <= 3 {
        counter += 1;
    }

    println!("{x} {counter}");

    let arr: [u8; 3] = [1, 2, 3];
    for element in arr {
        println!("{element}");
    }

    for n in (1..4).rev() {
        println!("{n}");
    }
}

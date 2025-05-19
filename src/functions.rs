fn main() {
    println!("hello");
    let b = another_func(5, 1);
    println!("{b}");

    let y = {
        let x = 1;
        x + 1
    };
    
    println!("y: {y}");
}

fn another_func(x: u8, y: u8) -> u8 {
    x + y
}

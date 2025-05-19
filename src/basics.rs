fn main() {
    let y = 1;
    let mut x = 5;
    const SIGNAL: u8 = 10;
    println!("x: {x}");
    x = 6;

    // Shadowing
    {
        x = x + 1;
        println!("x: {x}");
    }

    println!("x: {x}");
    println!("y: {y}");
    println!("signal: {SIGNAL}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
}

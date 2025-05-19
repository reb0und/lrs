fn main() {
    // Scalar primitives
    let guess: u8 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let a = 1 + 2;
    let b = 1.0 - 42.1;
    let c = 12 * 5;
    
    // If doing some similar operation and receing ints, should convert to float
    let truncated = 32 / 12;
    let d = 8 / 5;

    let e = 10 % 5;

    let tf: bool = true;

    let letter: char = 'x';
    
    println!("tf: {tf}, letter: {letter}");

    // Compound primitives
    // Tuples

    let tup: (u8, char) = (1, 'a');
    let (x, y) = tup;
    let x = tup.0;
    let y = tup.1;
    println!("x: {x}, y: {y}");

    // Arrays
    let l: [i8; 4] = [1, 2, 3, 4];
    
    let f = l[0];
    println!("{f}");
}

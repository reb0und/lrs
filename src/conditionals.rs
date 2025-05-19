fn main() {
    let num = 6;

    if num < 5 {
        println!("num is less than 5");
    } else if num % 5 == 0 {
        println!("num is divisible by 5");
    } else {
        println!("num is >= 5 and not divisible by 5")
    }

    let a = if true { 1 } else { 2 };
}

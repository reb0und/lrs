fn main() {
    let s = "hi";

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s = {s}");


    // Scope/ownership example
    {
        let a = String::new();
        // a's memory is returned to allocator

        // This is called automatically when a goes out of scope;
        drop(a);
    }

    // bind 5 to x
    let x = 5;
    // bind a copy of x to y
    let y = x;

    // Create a string
    let s1 = String::from("hello");
    let s2 = s1;

    // Will panic, borrow of moved value
    // println!("{s1}");

    let mut b = String::from("abc");
    // Now nothing refers to the origin value "abc" on the heap and it goes out of scope
    b = String::from("xyz");
    
    println!("b = {b}");

    // Deep copy/clone example
    let b2 = b.clone();
    println!("b2 = {b2}");

    let ss = String::from("hello");
    takes_ownership(ss);

    let f = 5;
    makes_copy(f);

    println!("{f}");

    let so = gives_ownership();

    let sf = String::from("hello");

    let tb = takes_and_gives_back(s2);

    let (sf, len) = calculate_length(sf);

    println!("sf = {sf}, len = {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: u8) {
    println!("{some_int}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

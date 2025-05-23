fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s1 = data.to_string();
    let s1 = "initial contents".to_string();
    let mut s1 = String::from("initial contents");

    s1.push_str("more");
    s1.push('a');
    println!("{s1}");

    let s1 = String::from("hi");
    let s2 = String::from("w");

    let s3 = s1 + &s2;

    let s4 = format!("{s3}-{s2}");

    for c in s4.chars() {
        println!("{c}");
    }

    for b in s4.bytes() {
        println!("{b}");
    }
}

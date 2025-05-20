fn main() {
    let mut s = String::from("hello world");

    // 'hello'
    let h = &s[0..5];

    let x = first_word(&s);

    // Cannot create an mutable reference will h is still alive, cannot have mutable
    // reference while an immutable reference exists
    //s.clear();

    println!("{h}");

    let a = [1, 2, 3];

    let slice = &a[..1];

    assert_eq!(slice, &[1])
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Return String slice from 0:i -> &s[..i]
            return &s[..i];
        }
    }

    // Return whole string
    &s[..]
}

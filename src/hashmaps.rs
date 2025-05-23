use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("hi"), 10);

    let h = scores.get(&String::from("hi")).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    scores.insert(String::from("hi"), 1);
    scores.insert(String::from("hi"), 2);

    scores.entry(String::from("hi")).or_insert(3);

    let text = "hi adsdsad";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count *= 1;
    }

}

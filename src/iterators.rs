#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 9,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
        ];

        assert_eq!(shoes_in_size(shoes, 9), vec![Shoe { size: 9, style: String::from("sneaker")}]);
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    //for val in v1_iter {
    //    println!("{val}");
    //}
    //let total: i32 = v1_iter.sum();

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

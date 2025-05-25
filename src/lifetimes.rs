use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("{ann}");
    if x.len() > y.len() { x } else { y }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> u8 {
        3
    }

    fn announce(&self, announcement: &str) -> &str {
        self.part
    }
}

fn main() {
    let s: &'static str = "xyz";
    let n = "ads";
    let i = ImportantExcerpt {
        part: n,
    };
    //let string1 = String::from("ayx");
    //let string2 = "xyz";
    //
    //let result = longest(string1.as_str(), string2);
    //println!("{result}");
    //
    //let s1 = String::from("ads");
    //
    //let result;
    //{
    //    let s2 = String::from("from");
    //    result = longest(s1.as_str(), s2.as_str());
    //}
    //println!("{result}");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hi")),
        SpreadsheetCell::Float(1.1),
    ];
    {
        let v4 = vec![1, 2, 3];

    } // v4 goes out of scope here, and is freed

    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("{third}");

    let third: Option<&i32> = v2.get(2);

    match third {
        Some(&third) => println!("{third}"),
        None => println!("none"),
    }


    v1.push(8);
    let a = &v1[0];

    // Mutable borrow occurs here during the lifetime of an immutable borrow which is not
    // allowed by rust compiler
    //v1.push(1);
    println!("{a}");
    // Panics
    //&v1[100];
    v1.get(100);

    for i in &mut v2 {
        *i += 1;
    }
}
